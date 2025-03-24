use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::Object;

mod imp {

    use std::rc::Rc;

    use crate::components::device_listing::DeviceListing;
    use crate::{BLUETOOTH, bluetooth::Device};
    use adw::glib::subclass::InitializingObject;
    use adw::subclass::prelude::*;
    use gtk::glib::clone;
    use gtk::glib::variant::ObjectPath;
    use gtk::{
        CompositeTemplate,
        gio::ListStore,
        glib::{self, object::Cast},
        subclass::widget::WidgetImpl,
    };

    use super::ConnectDialog;

    #[derive(CompositeTemplate)]
    #[template(resource = "/io/github/andreibachim/bike/ui/connect_dialog.ui")]
    pub struct ConnectDialogPrivate {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        device_list: TemplateChild<gtk::ListBox>,
        available_devices: ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectDialogPrivate {
        const NAME: &str = "ConnectDialog";
        type Type = super::ConnectDialog;
        type ParentType = adw::Dialog;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl ConnectDialogPrivate {
        #[template_callback]
        fn showing_find_page(slf: ConnectDialog) {
            log::debug!("Starting scan for new devices");
            let add_device_callback = Rc::new(clone!(
                #[weak(rename_to = imp)]
                slf.imp(),
                move |device| imp.add_new_device(device)
            ));
            let remove_device_callback = Rc::new(clone!(
                #[weak(rename_to = imp)]
                slf.imp(),
                move |object_path| imp.remove_device(object_path)
            ));
            BLUETOOTH.start_scanning_for_devices(add_device_callback, remove_device_callback);
        }

        #[template_callback]
        fn hiding_find_page() {
            log::debug!("Stopping scan for new devices");
            if BLUETOOTH.stop_scanning_for_devices().is_err() {
                todo!("Implement logic for error cases here")
            }
        }

        fn add_new_device(&self, device: Device) {
            device.register_property_listener();
            self.available_devices.append(&device);
        }

        fn remove_device(&self, object_path: ObjectPath) {
            log::debug!("Should remove device: {:#?}", object_path);
            self.available_devices.retain(|device| {
                if let Some(device) = device.downcast_ref::<Device>() {
                    if device
                        .object_path()
                        .eq_ignore_ascii_case(object_path.as_str())
                    {
                        device.unregister_property_listener();
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
        }
    }

    impl Default for ConnectDialogPrivate {
        fn default() -> Self {
            Self {
                available_devices: ListStore::new::<Device>(),
                device_list: Default::default(),
                navigation_view: Default::default(),
            }
        }
    }

    impl ObjectImpl for ConnectDialogPrivate {
        fn constructed(&self) {
            self.parent_constructed();
            self.device_list
                .bind_model(Some(&self.available_devices), |device| {
                    match device.downcast_ref::<Device>() {
                        Some(device) => {
                            let device_listing = DeviceListing::new(device);
                            device_listing.into()
                        }
                        None => adw::Bin::new().into(),
                    }
                });
        }
    }
    impl WidgetImpl for ConnectDialogPrivate {}
    impl AdwDialogImpl for ConnectDialogPrivate {}
}

gtk::glib::wrapper! {
    pub struct ConnectDialog(ObjectSubclass<imp::ConnectDialogPrivate>)
        @extends adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl ConnectDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn load_details(&self) {
        self.imp()
            .navigation_view
            .push_by_tag("device-details-page");
    }

    pub fn skip_to_device_details_page(&self) {
        self.imp().navigation_view.set_animate_transitions(false);
        self.load_details();
        self.imp().navigation_view.set_animate_transitions(true);
    }
}

impl Default for ConnectDialog {
    fn default() -> Self {
        Self::new()
    }
}
