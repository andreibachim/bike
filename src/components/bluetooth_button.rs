use gtk::glib::Object;

mod imp {
    use std::cell::Cell;

    use adw::prelude::ObjectExt;
    use adw::subclass::prelude::*;
    use adw::{glib::subclass::InitializingObject, prelude::AdwDialogExt};
    use gtk::glib::types::StaticType;
    use gtk::prelude::WidgetExt;
    use gtk::{
        CompositeTemplate,
        glib::{self, Properties, clone},
        subclass::widget::WidgetImpl,
    };

    use crate::{BLUETOOTH, components::connect_dialog::ConnectDialog};

    use super::BluetoothButton;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::BluetoothButton)]
    #[template(resource = "/io/github/andreibachim/bike/ui/bluetooth_button.ui")]
    pub struct BluetoothButtonPrivate {
        #[property(name="state", get, set, type = State, builder(State::default()))]
        state: Cell<State>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BluetoothButtonPrivate {
        const NAME: &str = "BluetoothButton";
        type Type = super::BluetoothButton;
        type ParentType = gtk::Button;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl BluetoothButtonPrivate {
        #[template_callback]
        fn clicked(slf: BluetoothButton) {
            match slf.state() {
                State::Disabled => (),
                State::PoweredOff => (),
                State::Disconnected => {
                    let connect_dialog = ConnectDialog::new();
                    connect_dialog
                        .present(slf.ancestor(adw::ApplicationWindow::static_type()).as_ref());
                }
                State::Connected => {}
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BluetoothButtonPrivate {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj()
                .bind_property("state", self.obj().as_ref(), "icon-name")
                .sync_create()
                .transform_to(|_, state: State| -> Option<&str> {
                    match state {
                        State::Disabled => Some("bluetooth-hardware-disabled-symbolic"),
                        State::PoweredOff => Some("bluetooth-disabled-symbolic"),
                        State::Disconnected => Some("bluetooth-disconnected-symbolic"),
                        State::Connected => Some("bluetooth-active-symbolic"),
                    }
                })
                .build();

            self.obj()
                .bind_property("state", self.obj().as_ref(), "css-classes")
                .sync_create()
                .transform_to(|_, state: State| -> Option<&[&str]> {
                    match state {
                        State::Disabled => Some(&["destructive-action"]),
                        State::PoweredOff => Some(&["error"]),
                        State::Disconnected => Some(&["suggested-action"]),
                        State::Connected => Some(&["success"]),
                    }
                })
                .build();

            self.obj()
                .bind_property("state", self.obj().as_ref(), "tooltip-text")
                .sync_create()
                .transform_to(|_, state: State| -> Option<&str> {
                    match state {
                        State::Disabled => Some("No bluetooth adapter detected"),
                        State::PoweredOff => Some("Bluetooth is turned off"),
                        _ => Some(""),
                    }
                })
                .build();

            match BLUETOOTH.is_adapter_powered() {
                Ok(value) => match value {
                    true => self.obj().set_state(State::Disconnected),
                    false => self.obj().set_state(State::PoweredOff),
                },
                Err(_) => {
                    self.obj().set_state(State::Disabled);
                }
            }
            BLUETOOTH.start_adapter_monitoring(clone!(
                #[strong(rename_to = obj)]
                self.obj(),
                move |value| {
                    match value {
                        true => obj.set_state(State::Disconnected),
                        false => obj.set_state(State::PoweredOff),
                    }
                }
            ));
        }
    }
    impl WidgetImpl for BluetoothButtonPrivate {}
    impl ButtonImpl for BluetoothButtonPrivate {}

    #[derive(Default, Debug, PartialEq, Eq, Copy, Clone, glib::Enum)]
    #[enum_type(name = "BluetoothButtonStatus")]
    pub enum State {
        #[default]
        Disabled,
        PoweredOff,
        Disconnected,
        Connected,
    }
}

gtk::glib::wrapper! {
    pub struct BluetoothButton(ObjectSubclass<imp::BluetoothButtonPrivate>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl BluetoothButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for BluetoothButton {
    fn default() -> Self {
        Self::new()
    }
}
