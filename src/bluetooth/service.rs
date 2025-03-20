use std::collections::HashMap;

use gtk::{
    gio::{BusType, Cancellable, DBusCallFlags, DBusConnection, DBusError, DBusSignalFlags},
    glib::{Variant, VariantTy, variant::ToVariant},
};

pub struct BluetoothService {
    connection: Result<DBusConnection, gtk::glib::Error>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let connection = gtk::gio::bus_get_sync(BusType::System, Cancellable::NONE);
        //let proxy = DBusProxy::for_bus_sync(
        //    gtk::gio::BusType::System,
        //    DBusProxyFlags::NONE,
        //    None,
        //    "org.bluez",
        //    "/org/bluez/hci0",
        //    "org.freedesktop.DBus.Properties",
        //    Cancellable::NONE,
        //)
        //.expect("Could not create DBus adapter proxy object");
        //
        //let powered = proxy.call_sync(
        //    "Get",
        //    Some(&("org.bluez.Adapter1", "Powered").to_variant()),
        //    DBusCallFlags::NONE,
        //    300,
        //    Cancellable::NONE,
        //);

        //log::debug!("{:#?}", powered);

        Self { connection }
    }

    pub fn is_valid(&self) -> bool {
        self.connection.is_ok()
    }

    pub fn is_adapter_powered(&self) -> Result<bool, gtk::glib::Error> {
        let powered_variant = &self.connection.clone()?.call_sync(
            Some("org.bluez"),
            "/org/bluez/hci0",
            "org.freedesktop.DBus.Properties",
            "Get",
            Some(&("org.bluez.Adapter1", "Powered").to_variant()),
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
        if let Ok(connection) = &self.connection {
            connection.signal_subscribe(
                Some("org.bluez"),
                Some("org.freedesktop.DBus.Properties"),
                Some("PropertiesChanged"),
                Some("/org/bluez/hci0"),
                Some("org.bluez.Adapter1"),
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
