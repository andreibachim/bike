use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;
    use gtk::{CompositeTemplate, glib::subclass::InitializingObject};

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/github/andreibachim/bike/ui/device_details_page.ui")]
    pub struct DeviceDetailsPagePrivate {}

    #[glib::object_subclass]
    impl ObjectSubclass for DeviceDetailsPagePrivate {
        const NAME: &'static str = "DeviceDetailsPage";
        type Type = super::DeviceDetailsPage;
        type ParentType = adw::NavigationPage;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl DeviceDetailsPagePrivate {
        #[template_callback]
        fn showing() {}
        #[template_callback]
        fn hiding() {}
    }

    impl ObjectImpl for DeviceDetailsPagePrivate {}
    impl WidgetImpl for DeviceDetailsPagePrivate {}
    impl NavigationPageImpl for DeviceDetailsPagePrivate {}
}

glib::wrapper! {
    pub struct DeviceDetailsPage(ObjectSubclass<imp::DeviceDetailsPagePrivate>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
