use bike_bt::BluetoothStatus;
use relm4::{
    adw::prelude::AdwDialogExt,
    gtk::{
        glib::clone,
        prelude::{ButtonExt, WidgetExt},
    },
    Component, ComponentController, ComponentParts, Controller, MessageBroker, RelmWidgetExt,
};

use crate::state_manager::StateManagerInput;

use crate::components::connect_dialog::ConnectDialog;

#[derive(Debug)]
pub enum AdapterStateInput {
    ChangeStatus(BluetoothStatus),
    Clicked,
}
pub static ADAPTER_STATE_BROKER: MessageBroker<AdapterStateInput> = MessageBroker::new();

pub struct BluetoothButton {
    status: BluetoothStatus,
    #[allow(dead_code)]
    connect_dialog: Controller<ConnectDialog>,
}

pub struct BluetoothStatusWidgets {
    button: relm4::gtk::Button,
}

impl Component for BluetoothButton {
    type Input = AdapterStateInput;
    type Output = ();
    type Init = ();
    type Root = relm4::gtk::Button;
    type Widgets = BluetoothStatusWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        relm4::gtk::Button::builder()
            .focusable(false)
            .icon_name("bluetooth-hardware-disabled")
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        crate::brokers::STATE_MANAGER.send(StateManagerInput::RegisterAdapterListener);
        let connect_dialog = ConnectDialog::builder().launch(()).detach();

        root.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AdapterStateInput::Clicked);
            }
        ));

        let model = BluetoothButton {
            status: BluetoothStatus::Unavailable,
            connect_dialog,
        };

        let widgets = BluetoothStatusWidgets { button: root };

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        _sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            AdapterStateInput::ChangeStatus(bluetooth_status) => self.status = bluetooth_status,
            AdapterStateInput::Clicked => match self.status {
                BluetoothStatus::Disconnected => {
                    let widget = self.connect_dialog.widget();
                    let window = root.toplevel_window();
                    widget.present(window.as_ref());
                }
                _ => {
                    println!("{:#?}", self.status);
                }
            },
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _: relm4::ComponentSender<Self>) {
        match self.status {
            BluetoothStatus::Unavailable => {
                widgets.button.set_sensitive(false);
                widgets.button.set_css_classes(&["flat", "error"]);
                widgets
                    .button
                    .set_tooltip_text(Some("Bluetooth is not available"));
                widgets.button.set_icon_name("bluetooth-hardware-disabled");
            }
            BluetoothStatus::PoweredOff => {
                widgets.button.set_sensitive(false);
                widgets.button.set_css_classes(&["flat", "error"]);
                widgets
                    .button
                    .set_tooltip_text(Some("Bluetooth is turned off"));
                widgets.button.set_icon_name("bluetooth-disabled");
            }
            BluetoothStatus::Disconnected => {
                widgets.button.set_sensitive(true);
                widgets.button.set_css_classes(&["suggested-action"]);
                widgets.button.set_tooltip_text(None);
                widgets.button.set_icon_name("bluetooth-disconnected");
            }
            BluetoothStatus::Connected => {
                widgets.button.set_tooltip_text(None);
                widgets.button.set_css_classes(&["flat", "success"]);
            }
        }
    }
}
