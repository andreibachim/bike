mod imp {
    use adw::subclass::prelude::*;
    use gtk::{
        glib::{self, object::Cast},
        prelude::GtkWindowExt,
    };

    use crate::components::window::Window;

    #[derive(Default)]
    pub struct AppPrivate;

    #[glib::object_subclass]
    impl ObjectSubclass for AppPrivate {
        const NAME: &'static str = "App";
        type Type = super::App;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for AppPrivate {}
    impl WidgetImpl for AppPrivate {}
    impl ApplicationImpl for AppPrivate {
        fn activate(&self) {
            let window = Window::new(&self.obj().clone().upcast::<adw::Application>());
            window.present();
        }
    }
    impl GtkApplicationImpl for AppPrivate {}
    impl AdwApplicationImpl for AppPrivate {}
}

use gtk::glib::{self, Object};
use gtk::prelude::ApplicationExtManual;
glib::wrapper! {
    pub struct App(ObjectSubclass<imp::AppPrivate>)
    @extends gtk::gio::Application, gtk::Application, adw::Application,
    @implements gtk::gio::ActionMap, gtk::gio::ActionGroup;
}

impl App {
    pub fn new() -> Self {
        Object::builder()
            .property("application_id", "io.github.andreibachim.bike")
            .build()
    }

    pub fn start(&self) -> glib::ExitCode {
        self.run()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
