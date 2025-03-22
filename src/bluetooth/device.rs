mod imp {
    use std::cell::RefCell;

    use adw::prelude::ObjectExt;
    use adw::subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass};
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
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DevicePrivate {
        const NAME: &'static str = "Device";
        type Type = super::Device;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DevicePrivate {}
}

use std::fmt::Display;

use gtk::glib::{self, Object};

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
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.object_path(),
            self.name(),
            self.paired(),
            self.connected(),
            self.rssi()
        )
    }
}
