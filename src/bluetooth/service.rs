use std::collections::HashMap;

use gtk::{
    gio::{BusType, Cancellable, DBusCallFlags, DBusConnection, DBusError, DBusSignalFlags},
    glib::{Variant, VariantTy, variant::ToVariant},
};

const BLUEZ_BUS_NAME: Option<&str> = Some("org.bluez");
const ADAPTER_OBJECT_PATH: Option<&str> = Some("/org/bluez/hci0");
const ADAPTER_INTERFACE: Option<&str> = Some("org.bluez.Adapter1");
const PROPERTIES_INTERFACE: Option<&str> = Some("org.freedesktop.DBus.Properties");

pub struct BluetoothService {
    connection: Result<DBusConnection, gtk::glib::Error>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let connection = gtk::gio::bus_get_sync(BusType::System, Cancellable::NONE);
        Self { connection }
    }

    pub fn is_valid(&self) -> bool {
        self.connection.is_ok()
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
            ADAPTER_OBJECT_PATH.expect("Adapter object path is not defined"),
            PROPERTIES_INTERFACE.expect("Properties interface name is not defined"),
            "Get",
            Some(
                &(
                    ADAPTER_INTERFACE.expect("Adapter interface name is not defined"),
                    "Powered",
                )
                    .to_variant(),
            ),
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
                PROPERTIES_INTERFACE,
                Some("PropertiesChanged"),
                ADAPTER_OBJECT_PATH,
                ADAPTER_INTERFACE,
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
}

impl Default for BluetoothService {
    fn default() -> Self {
        Self::new()
    }
}
