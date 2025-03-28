mod imp {

    use adw::subclass::prelude::{
        ActionRowImpl, ObjectImpl, ObjectSubclass, PreferencesRowImpl, WidgetClassExt,
    };
    use gtk::glib::object::CastNone;
    use gtk::glib::subclass::InitializingObject;
    use gtk::glib::types::StaticType;
    use gtk::prelude::WidgetExt;
    use gtk::subclass::widget::{
        CompositeTemplateCallbacksClass, CompositeTemplateClass, CompositeTemplateInitializingExt,
    };
    use gtk::{CompositeTemplate, TemplateChild};
    use gtk::{
        glib::{self},
        subclass::{prelude::ListBoxRowImpl, widget::WidgetImpl},
    };

    use crate::components::connect_dialog::ConnectDialog;

    use super::DeviceListing;

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
    impl DeviceListingPrivate {
        #[template_callback]
        fn connect(slf: DeviceListing) {
            slf.ancestor(ConnectDialog::static_type())
                .and_downcast()
                .inspect(|connect_dialog: &ConnectDialog| connect_dialog.load_details());
        }
    }

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
        ClosureExpression::new::<String>(
            [
                &device.property_expression("connected"),
                &device.property_expression("rssi"),
            ],
            closure!(|_: gtk::Image, connected: bool, rssi: i32| {
                if connected {
                    "go-next-symbolic"
                } else {
                    match rssi {
                        value if (-119..=-90).contains(&value) => {
                            "network-cellular-signal-weak-symbolic"
                        }
                        value if (-89..=-60).contains(&value) => {
                            "network-cellular-signal-ok-symbolic"
                        }
                        value if (-59..=-30).contains(&value) => {
                            "network-cellular-signal-good-symbolic"
                        }
                        value if (-29..=0).contains(&value) => {
                            "network-cellular-signal-excellent-symbolic"
                        }
                        _ => "network-cellular-offline-symbolic",
                    }
                }
            }),
        )
        .bind(
            &slf.imp().signal_icon.get(),
            "icon-name",
            Some(&slf.imp().signal_icon.get()),
        );

        slf
    }
}
