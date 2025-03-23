mod imp {

    use adw::subclass::prelude::{
        ActionRowImpl, ObjectImpl, ObjectSubclass, PreferencesRowImpl, WidgetClassExt,
    };
    use gtk::glib::subclass::InitializingObject;
    use gtk::subclass::widget::{
        CompositeTemplateCallbacksClass, CompositeTemplateClass, CompositeTemplateInitializingExt,
    };
    use gtk::{CompositeTemplate, TemplateChild};
    use gtk::{
        glib::{self},
        subclass::{prelude::ListBoxRowImpl, widget::WidgetImpl},
    };

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/andreibachim/bike/ui/device_listing.ui")]
    pub struct DeviceListingPrivate {
        #[template_child]
        pub signal_icon: TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DeviceListingPrivate {
        const NAME: &'static str = "DeviceListing";
        type Type = super::DeviceListing;
        type ParentType = adw::ActionRow;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl DeviceListingPrivate {}

    impl ObjectImpl for DeviceListingPrivate {}
    impl WidgetImpl for DeviceListingPrivate {}
    impl ListBoxRowImpl for DeviceListingPrivate {}
    impl PreferencesRowImpl for DeviceListingPrivate {}
    impl ActionRowImpl for DeviceListingPrivate {}
}

use adw::subclass::prelude::{ObjectSubclass, ObjectSubclassIsExt};
use gtk::{
    ClosureExpression,
    glib::{self, Object, closure, object::ObjectExt},
    prelude::GObjectPropertyExpressionExt,
};

use crate::bluetooth::Device;

glib::wrapper! {
    pub struct DeviceListing(ObjectSubclass<imp::DeviceListingPrivate>)
        @extends adw::ActionRow, gtk::Widget,
        @implements gtk::Actionable, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceListing {
    pub fn new(device: &Device) -> Self {
        log::debug!("Device found: {device}");
        let slf: Self = Object::builder().build();

        //Bind title
        device
            .bind_property("name", &slf, "title")
            .sync_create()
            .build();

        //Bind subtitle
        ClosureExpression::new::<String>(
            [
                &device.property_expression("paired"),
                &device.property_expression("connected"),
            ],
            closure!(|_: <imp::DeviceListingPrivate as ObjectSubclass>::Type,
                      paired: bool,
                      connected: bool| {
                if connected {
                    "Connected"
                } else if paired {
                    "Disconnected"
                } else {
                    "Not Set Up"
                }
            }),
        )
        .bind(&slf, "subtitle", Some(&slf));

        //Bind icon
        device
            .bind_property("rssi", &slf.imp().signal_icon.get(), "icon-name")
            .sync_create()
            .transform_to(|_, rssi: i32| -> Option<&str> {
                Some(match rssi {
                    value if (-119..=-90).contains(&value) => {
                        "network-cellular-signal-weak-symbolic"
                    }
                    value if (-89..=-60).contains(&value) => "network-cellular-signal-ok-symbolic",
                    value if (-59..=-30).contains(&value) => {
                        "network-cellular-signal-good-symbolic"
                    }
                    value if (-29..=0).contains(&value) => {
                        "network-cellular-signal-excellent-symbolic"
                    }
                    _ => "network-cellular-offline-symbolic",
                })
            })
            .build();

        slf
    }
}
