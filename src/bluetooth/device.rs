mod imp {
    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::glib;
    #[derive(Debug, Default)]
    pub struct DevicePrivate {}

    #[glib::object_subclass]
    impl ObjectSubclass for DevicePrivate {
        const NAME: &'static str = "Device";
        type Type = super::Device;
    }
    impl ObjectImpl for DevicePrivate {}
}

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct Device(ObjectSubclass<imp::DevicePrivate>);
}

impl Device {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new() 
    }
}
