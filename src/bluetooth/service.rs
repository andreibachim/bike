use std::collections::HashMap;

use gtk::{
    gio::{BusType, Cancellable, DBusCallFlags, DBusConnection, DBusError, DBusSignalFlags},
    glib::{
        Variant, VariantTy,
        variant::{FromVariant, ObjectPath, ToVariant},
    },
};

const BLUEZ_BUS_NAME: Option<&str> = Some("org.bluez");
const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
const PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";
const OBJECT_MANAGER_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";

pub struct BluetoothService {
    connection: Result<DBusConnection, gtk::glib::Error>,
    adapter_index: usize,
    adapters: Vec<ObjectPath>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let connection = gtk::gio::bus_get_sync(BusType::System, Cancellable::NONE);
        let mut slf = Self {
            connection,
            adapter_index: 0,
            adapters: vec![],
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

    pub fn start_device_monitoring(&self) {
        if let Ok(connection) = &self.connection {
            //(oa{sa{sv}})
            connection.signal_subscribe(
                BLUEZ_BUS_NAME,
                Some(OBJECT_MANAGER_INTERFACE),
                Some("InterfacesAdded"),
                Some("/"),
                None,
                DBusSignalFlags::NONE,
                |_, _, _, _, _, value| {
                    let value = value
                        .get::<(ObjectPath, HashMap<String, HashMap<String, Variant>>)>()
                        .unwrap_or_else(|| {
                            (
                                ObjectPath::from_variant(&"/org/bluez".to_variant())
                                    .expect("Mock object path could not be created"),
                                HashMap::new(),
                            )
                        });
                    if value.1.contains_key("org.bluez.Device1") {
                        let device_data = value.1.get("org.bluez.Device1").unwrap();
                        //Name
                        let name = device_data
                            .get("Name")
                            .and_then(|variant| variant.get::<String>());
                        if name.is_none() {
                            return;
                        };
                        //RSSI
                        let rssi = device_data
                            .get("RSSI")
                            .and_then(|variant| variant.get::<i16>())
                            .unwrap_or_else(|| -200i16);

                        device_data.get("Name").inspect(|name| {
                            log::debug!("Device {name} found with RSSI '{rssi}'");
                        });
                    };
                },
            );

            connection.signal_subscribe(
                BLUEZ_BUS_NAME,
                Some(OBJECT_MANAGER_INTERFACE),
                Some("InterfacesRemoved"),
                Some("/"),
                None,
                DBusSignalFlags::NONE,
                |_, _, _, _, _, value| {
                    //log::debug!("{:#?}", value);
                },
            );
        }
    }

    pub fn start_scanning_for_devices(&self) {
        if let Ok(connection) = &self.connection {
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

    pub fn stop_scanning_for_devices(&self) {
        if let Ok(connection) = &self.connection {
            let _ = connection.call_sync(
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
            );
        }
    }
}

impl Default for BluetoothService {
    fn default() -> Self {
        Self::new()
    }
}
