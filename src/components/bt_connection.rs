use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct BtConnection(ObjectSubclass<imp::BtConnection>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl BtConnection {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use gtk::prelude::ButtonExt;

    // Object holding the state
    #[derive(Default)]
    pub struct BtConnection {
        icon_name: String,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for BtConnection {
        const NAME: &'static str = "BtConnection";
        type Type = super::BtConnection;
        type ParentType = gtk::Button;
        fn new() -> Self {
            glib::spawn_future_local(async {
                tokio::spawn(async {
                    println!("{:#?}", bluer::Session::new().await.unwrap().adapter_names().await.unwrap());
                });
            });

            Self {
                icon_name: "bluetooth".to_owned()
            }
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for BtConnection {
        fn constructed(&self) {
            //self.parent_constructed();
            let obj = self.obj();
            obj.set_icon_name(&self.icon_name);
        }
    }
    // Trait shared by all widgets
    impl WidgetImpl for BtConnection {}
    // Trait shared by all buttons
    impl ButtonImpl for BtConnection {
        fn clicked(&self) {
            println!("Hello, world!");
        }
    }
}

