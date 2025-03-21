use std::collections::HashMap;

use gtk::{
    gio::{BusType, Cancellable, DBusCallFlags, DBusConnection, DBusError, DBusSignalFlags},
    glib::{
        Variant, VariantTy,
        variant::{ObjectPath, ToVariant},
    },
};

const BLUEZ_BUS_NAME: Option<&str> = Some("org.bluez");
const ADAPTER_OBJECT_PATH: &str = "/org/bluez/hci0";
const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
const PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";

pub struct BluetoothService {
    connection: Result<DBusConnection, gtk::glib::Error>,
    adapter_object_path: Option<String>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let connection = gtk::gio::bus_get_sync(BusType::System, Cancellable::NONE);
        let mut slf = Self { connection, adapter_object_path: None };
        slf.adapter_object_path = slf.get_adapters().first().map(|object_path| object_path.to_string());
        slf
    }

    pub fn is_valid(&self) -> bool {
        self.connection.is_ok() && self.adapter_object_path.is_some()
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
            ADAPTER_OBJECT_PATH,
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
                Some(ADAPTER_OBJECT_PATH),
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
                    "org.freedesktop.DBus.ObjectManager",
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
}

impl Default for BluetoothService {
    fn default() -> Self {
        Self::new()
    }
}
