use std::{
    collections::HashMap, rc::Rc, sync::{Arc, Mutex}
};

use gtk::{
    gio::{
        BusType, Cancellable, DBusCallFlags, DBusConnection, DBusError, DBusSignalFlags,
        SignalSubscriptionId,
    },
    glib::{
        Variant, VariantTy,
        variant::{FromVariant, ObjectPath, ToVariant},
    },
};

use super::Device;

const BLUEZ_BUS_NAME: Option<&str> = Some("org.bluez");
const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
const DEVICE_INTERFACE: &str = "org.bluez.Device1";
const PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";
const OBJECT_MANAGER_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";

pub struct BluetoothService {
    connection: Result<DBusConnection, gtk::glib::Error>,
    adapter_index: usize,
    adapters: Vec<ObjectPath>,
    interface_added_sub_id: Arc<Mutex<Option<SignalSubscriptionId>>>,
    interface_removed_sub_id: Arc<Mutex<Option<SignalSubscriptionId>>>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let connection = gtk::gio::bus_get_sync(BusType::System, Cancellable::NONE);
        let mut slf = Self {
            connection,
            adapter_index: 0,
            adapters: vec![],
            interface_added_sub_id: Arc::new(Mutex::new(None)),
            interface_removed_sub_id: Arc::new(Mutex::new(None)),
        };
        slf.adapters.append(&mut slf.get_adapters());
        slf
    }

    pub fn is_valid(&self) -> bool {
        self.connection.is_ok() && !self.adapters.is_empty()
    }

    pub fn is_adapter_powered(&self) -> Result<bool, gtk::glib::Error> {
        if !self.is_valid() {
            return Err(DBusError::new_for_dbus_error(
                "No bluetooth connection",
                "Bluetooth connection is not active",
            ));
        }

        let powered_variant = &self.connection.clone()?.call_sync(
            BLUEZ_BUS_NAME,
            self.adapters
                .get(self.adapter_index)
                .expect("At least one adapter is needed"),
            PROPERTIES_INTERFACE,
            "Get",
            Some(&(ADAPTER_INTERFACE, "Powered").to_variant()),
            Some(VariantTy::ANY),
            DBusCallFlags::NONE,
            300,
            Cancellable::NONE,
        )?;
        powered_variant
            .get::<(Variant,)>()
            .and_then(|(variant,)| variant.get::<bool>())
            .ok_or(DBusError::new_for_dbus_error(
                "Invalid property",
                "The 'Powered' property could not be read.",
            ))
    }

    pub fn start_adapter_monitoring<F>(&self, closure: F)
    where
        F: Fn(bool) + 'static,
    {
        if !self.is_valid() {
            return;
        };
        if let Ok(connection) = &self.connection {
            connection.signal_subscribe(
                BLUEZ_BUS_NAME,
                Some(PROPERTIES_INTERFACE),
                Some("PropertiesChanged"),
                self.adapters
                    .get(self.adapter_index)
                    .map(|object_path| object_path.as_str()),
                Some(ADAPTER_INTERFACE),
                DBusSignalFlags::NONE,
                move |_, _, _, _, _, value| {
                    let _ = value
                        .get::<(String, HashMap<String, Variant>, Vec<String>)>()
                        .map(|(_, map, _)| map)
                        .filter(|map| map.contains_key("Powered"))
                        .inspect(|map| {
                            if let Some(value) = map.get("Powered").and_then(|p| p.get::<bool>()) {
                                closure(value);
                            }
                        });
                },
            );
        }
    }

    fn get_adapters(&self) -> Vec<ObjectPath> {
        if let Ok(connection) = &self.connection {
            connection
                .call_sync(
                    BLUEZ_BUS_NAME,
                    "/",
                    OBJECT_MANAGER_INTERFACE,
                    "GetManagedObjects",
                    None,
                    Some(VariantTy::ANY),
                    DBusCallFlags::NONE,
                    3000,
                    Cancellable::NONE,
                )
                .unwrap_or(
                    (HashMap::<
                        ObjectPath,
                        HashMap<String, HashMap<String, Variant>>,
                    >::new(),)
                        .to_variant(),
                )
                .get::<(HashMap<ObjectPath, HashMap<String, HashMap<String, Variant>>>,)>()
                .unwrap_or((HashMap::new(),))
                .0
                .into_iter()
                .filter(|(_, v)| v.contains_key(ADAPTER_INTERFACE))
                .map(|(k, _)| k)
                .collect()
        } else {
            vec![]
        }
    }

    fn start_device_monitoring<F>(&self, connection: &DBusConnection, callback: Rc<F>) where F: Fn(Device) + 'static {
        let interface_added_sub_id = connection.signal_subscribe(
            BLUEZ_BUS_NAME,
            Some(OBJECT_MANAGER_INTERFACE),
            Some("InterfacesAdded"),
            Some("/"),
            None,
            DBusSignalFlags::NONE,
            move |_, _, _, _, _, value| {
                let value = value
                    .get::<(ObjectPath, HashMap<String, HashMap<String, Variant>>)>()
                    .unwrap_or_else(|| {
                        (
                            ObjectPath::from_variant(&"/org/bluez".to_variant())
                                .expect("Mock object path could not be created"),
                            HashMap::new(),
                        )
                    });
                if let Some(device_data) = value.1.get(DEVICE_INTERFACE) {
                    if let Some(device) = BluetoothService::device_from_data(value.0, device_data) {
                        callback(device);
                    }
                }
            },
        );

        match self.interface_added_sub_id.try_lock() {
            Ok(mut value) => {
                value.replace(interface_added_sub_id);
            }
            Err(error) => {
                log::error!(
                    "Could not acquire Interface Added lock. Unsubscribing from events. {error}",
                );
                connection.signal_unsubscribe(interface_added_sub_id);
            }
        };

        let interface_removed_sub_id = connection.signal_subscribe(
            BLUEZ_BUS_NAME,
            Some(OBJECT_MANAGER_INTERFACE),
            Some("InterfacesRemoved"),
            Some("/"),
            None,
            DBusSignalFlags::NONE,
            |_, _, _, _, _, _value| {
                //log::debug!("{:#?}", value);
            },
        );

        match self.interface_removed_sub_id.try_lock() {
            Ok(mut value) => {
                value.replace(interface_removed_sub_id);
            }
            Err(error) => {
                log::error!(
                    "Could not acquire Interface Removed lock. Unsubscribing from events. {error}"
                );
                connection.signal_unsubscribe(interface_removed_sub_id);
            }
        }
    }

    fn find_known_devices<F>(&self, connection: &DBusConnection, callback: Rc<F>) where F: Fn(Device) {
        if let Ok(devices) = connection.call_sync(
            BLUEZ_BUS_NAME,
            "/",
            "org.freedesktop.DBus.ObjectManager",
            "GetManagedObjects",
            None,
            Some(VariantTy::ANY),
            DBusCallFlags::NONE,
            3000,
            Cancellable::NONE,
        ) {
            let (devices,) = devices
                .get::<(HashMap<ObjectPath, HashMap<String, HashMap<String, Variant>>>,)>()
                .unwrap_or((HashMap::new(),));
            devices
                .into_iter()
                .filter(|(_, v)| v.contains_key(DEVICE_INTERFACE))
                .for_each(|(object_path, interfaces)| {
                    if let Some(device_data) = interfaces.get(DEVICE_INTERFACE) {
                        if let Some(device) =
                            BluetoothService::device_from_data(object_path, device_data)
                        {
                            callback(device);
                        }
                    }
                });
        }
    }

    fn device_from_data(
        object_path: ObjectPath,
        device_data: &HashMap<String, Variant>,
    ) -> Option<Device> {
        device_data
            .get("Name")
            .and_then(|variant| variant.get::<String>())
            .map(|name| {
                let rssi = device_data
                    .get("RSSI")
                    .and_then(|variant| variant.get::<i16>())
                    .unwrap_or(-200i16);
                let paired = device_data
                    .get("Paired")
                    .and_then(|variant| variant.get::<bool>())
                    .unwrap_or(false);
                let connected = device_data
                    .get("Connected")
                    .and_then(|variant| variant.get::<bool>())
                    .unwrap_or(false);
                Device::new(
                    name,
                    paired,
                    connected,
                    rssi.into(),
                    object_path.to_string(),
                )
            })
    }

    pub fn start_scanning_for_devices<F>(&self, callback: Rc<F>) where F: Fn(Device) + 'static {
        if let Ok(connection) = &self.connection {
            self.start_device_monitoring(connection, callback.clone());
            self.find_known_devices(connection, callback);
            let _ = connection.call_sync(
                BLUEZ_BUS_NAME,
                self.adapters
                    .get(self.adapter_index)
                    .expect("No adapter found"),
                ADAPTER_INTERFACE,
                "StartDiscovery",
                None,
                None,
                DBusCallFlags::NONE,
                3000,
                Cancellable::NONE,
            );
        }
    }

    pub fn stop_scanning_for_devices(&self) -> Result<(), ()> {
        if let Ok(connection) = &self.connection {
            self.unregister_interface_subscriptions()?;
            connection
                .call_sync(
                    BLUEZ_BUS_NAME,
                    self.adapters
                        .get(self.adapter_index)
                        .expect("No adapter found"),
                    ADAPTER_INTERFACE,
                    "StopDiscovery",
                    None,
                    None,
                    DBusCallFlags::NONE,
                    3000,
                    Cancellable::NONE,
                )
                .map_err(|_| ())?;
        }
        Ok(())
    }

    fn unregister_interface_subscriptions(&self) -> Result<(), ()> {
        if let Ok(connection) = &self.connection {
            if let Some(id) = self
                .interface_added_sub_id
                .try_lock()
                .map_err(|_| log::error!("Could not acquire lock for interface added sub is"))?
                .take()
            {
                connection.signal_unsubscribe(id);
            }

            if let Some(id) = self
                .interface_removed_sub_id
                .try_lock()
                .map_err(|_| log::error!("Could not acquire lock for interface removed sub id"))?
                .take()
            {
                connection.signal_unsubscribe(id);
            }
        }
        Ok(())
    }
}

impl Default for BluetoothService {
    fn default() -> Self {
        Self::new()
    }
}
