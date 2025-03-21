use gtk::glib::Object;

mod imp {
    use adw::glib::subclass::InitializingObject;
    use adw::subclass::prelude::*;
    use gtk::{
        CompositeTemplate,
        glib::{self},
        subclass::widget::WidgetImpl,
    };
    use crate::{bluetooth::Device, BLUETOOTH};

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/github/andreibachim/bike/ui/connect_dialog.ui")]
    pub struct ConnectDialogPrivate {
        #[template_child]
        device_list: TemplateChild<gtk::ListBox>,
        available_devices: Vec<Device>,
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
        fn showing_find_page() {
            log::debug!("Starting scan for new devices");
            BLUETOOTH.start_device_monitoring();
            BLUETOOTH.start_scanning_for_devices();
        }

        #[template_callback]
        fn hiding_find_page() {
            log::debug!("Stopping scan for new devices");
            BLUETOOTH.stop_scanning_for_devices();
        }
    }

    impl ObjectImpl for ConnectDialogPrivate {
        fn constructed(&self) {
            self.parent_constructed();
            let store = gtk::gio::ListStore::new::<Device>();
            store.extend_from_slice(&self.available_devices);
            self.device_list.bind_model(Some(&store), |_| {
                gtk::Label::new(Some("Hello, world")).into()
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
