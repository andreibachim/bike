use gtk::glib::Object;

mod imp {

    use std::rc::Rc;

    use crate::{BLUETOOTH, bluetooth::Device};
    use adw::glib::subclass::InitializingObject;
    use adw::subclass::prelude::*;
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
            let callback = Rc::new(move |device| slf.imp().add_new_device(device));
            BLUETOOTH.start_scanning_for_devices(callback);
        }

        #[template_callback]
        fn hiding_find_page() {
            log::debug!("Stopping scan for new devices");
            if BLUETOOTH.stop_scanning_for_devices().is_err() {
                todo!("Implement logic for error cases here")
            }
        }

        fn add_new_device(&self, device: Device) {
            self.available_devices.append(&device);
        }
    }

    impl Default for ConnectDialogPrivate {
        fn default() -> Self {
            Self {
                available_devices: ListStore::new::<Device>(),
                device_list: Default::default(),
            }
        }
    }

    impl ObjectImpl for ConnectDialogPrivate {
        fn constructed(&self) {
            self.parent_constructed();
            self.device_list
                .bind_model(Some(&self.available_devices), |device| {
                    match device.downcast_ref::<Device>() {
                        Some(device) => adw::ActionRow::builder()
                            .title(device.name())
                            .build()
                            .into(),
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
        @extends adw::Dialog, gtk::Widget;
}

impl ConnectDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ConnectDialog {
    fn default() -> Self {
        Self::new()
    }
}
