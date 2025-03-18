use gtk::glib::Object;

mod imp {
    use adw::glib::subclass::InitializingObject;
    use adw::subclass::prelude::*;
    use gtk::{
        CompositeTemplate,
        glib::{self},
        subclass::widget::WidgetImpl,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/github/andreibachim/bike/ui/connect_dialog.ui")]
    pub struct ConnectDialogPrivate {}

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectDialogPrivate {
        const NAME: &str = "ConnectDialog";
        type Type = super::ConnectDialog;
        type ParentType = adw::Dialog;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for ConnectDialogPrivate {}
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
