use gtk::glib::Object;

mod imp {
    use crate::BLUETOOTH;
    use adw::subclass::prelude::*;
    use gtk::{
        CompositeTemplate,
        glib::{self, subclass::InitializingObject},
        subclass::{prelude::ApplicationWindowImpl, widget::WidgetImpl, window::WindowImpl},
    };

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/io/github/andreibachim/bike/ui/window.ui")]
    pub struct WindowPrivate {
        #[template_child]
        missing_bluetooth_banner: TemplateChild<adw::Banner>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowPrivate {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WindowPrivate {
        fn constructed(&self) {
            self.parent_constructed();
            self.missing_bluetooth_banner
                .set_revealed(!BLUETOOTH.is_valid());
        }
    }
    impl WidgetImpl for WindowPrivate {}
    impl WindowImpl for WindowPrivate {}
    impl ApplicationWindowImpl for WindowPrivate {}
    impl AdwApplicationWindowImpl for WindowPrivate {}
}

gtk::glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::WindowPrivate>)
        @extends adw::ApplicationWindow ,gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
