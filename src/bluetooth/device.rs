mod imp {
    use std::cell::RefCell;

    use adw::prelude::ObjectExt;
    use adw::subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass};
    use gtk::gio::SignalSubscriptionId;
    use gtk::glib::{self, Properties};

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Device)]
    pub struct DevicePrivate {
        #[property(name = "name", get, set)]
        name: RefCell<String>,

        #[property(name = "paired", get, set)]
        paired: RefCell<bool>,

        #[property(name = "connected", get, set)]
        connected: RefCell<bool>,

        #[property(name = "rssi", get, set)]
        rssi: RefCell<i32>,

        #[property(name = "object-path", get, set)]
        object_path: RefCell<String>,

        pub rssi_sub_id: RefCell<Option<SignalSubscriptionId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DevicePrivate {
        const NAME: &'static str = "Device";
        type Type = super::Device;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DevicePrivate {}
}

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object, clone};
use std::fmt::Display;

use crate::BLUETOOTH;

glib::wrapper! {
    pub struct Device(ObjectSubclass<imp::DevicePrivate>);
}

impl Device {
    pub fn new(
        name: String,
        paired: bool,
        connected: bool,
        rssi: i32,
        object_path: String,
    ) -> Self {
        Object::builder()
            .property("name", name)
            .property("paired", paired)
            .property("connected", connected)
            .property("rssi", rssi)
            .property("object_path", object_path)
            .build()
    }

    pub fn register_property_listener(&self) {
        *self.imp().rssi_sub_id.borrow_mut() = BLUETOOTH.start_rssi_monitoring(
            self.object_path(),
            clone!(
                #[weak(rename_to=slf)]
                self.clone(),
                move |rssi| {
                    slf.set_rssi(rssi as i32);
                }
            ),
        );
    }

    pub fn unregister_property_listener(&self) {
        if let Some(sub_id) = self.imp().rssi_sub_id.borrow_mut().take() {
            BLUETOOTH.stop_rssi_monitoring(sub_id);
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Path: {}, Name: {}, Paired: {}, Connected: {}, RSSI: {}",
            self.object_path(),
            self.name(),
            self.paired(),
            self.connected(),
            self.rssi()
        )
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}
