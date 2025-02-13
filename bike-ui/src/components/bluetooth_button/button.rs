use bike_bt::BluetoothStatus;
use relm4::{
    adw::prelude::AdwDialogExt,
    gtk::{glib::clone, prelude::ButtonExt},
    Component, ComponentController, ComponentParts, Controller, MessageBroker, RelmWidgetExt,
    SimpleComponent,
};

use super::ConnectDialog;

pub static BLUETOOTH_STATUS_BROKER: MessageBroker<BluetoothButtonInput> = MessageBroker::new();

pub struct BluetoothButton {
    status: BluetoothStatus,
    connect_dialog: Controller<ConnectDialog>,
}

#[derive(Debug)]
pub enum BluetoothButtonInput {
    SetStatus(BluetoothStatus),
    Clicked,
}

pub struct BluetoothStatusWidgets {
    button: relm4::gtk::Button,
}

impl SimpleComponent for BluetoothButton {
    type Input = BluetoothButtonInput;
    type Output = ();
    type Init = BluetoothStatus;
    type Root = relm4::gtk::Button;
    type Widgets = BluetoothStatusWidgets;

    fn init_root() -> Self::Root {
        relm4::gtk::Button::builder()
            .icon_name("bluetooth-hardware-disabled")
            .build()
    }

    fn init(
        status: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let connect_dialog = ConnectDialog::builder().launch(root.clone()).detach();

        root.connect_clicked(clone!(move |_| sender.input(BluetoothButtonInput::Clicked)));
        let model = BluetoothButton {
            status,
            connect_dialog,
        };

        let widgets = BluetoothStatusWidgets { button: root };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            BluetoothButtonInput::SetStatus(bluetooth_status) => self.status = bluetooth_status,
            BluetoothButtonInput::Clicked => match self.status {
                BluetoothStatus::Disconnected => {
                    let widget = self.connect_dialog.widget();
                    let window = self.connect_dialog.model().owner.toplevel_window();
                    widget.present(window.as_ref());
                }
                _ => {
                    println!("{:#?}", self.status);
                }
            },
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::ComponentSender<Self>) {
        widgets.button.set_icon_name(match self.status {
            BluetoothStatus::Unavailable => "bluetooth-hardware-disabled",
            BluetoothStatus::PoweredOff => "bluetooth-disabled",
            BluetoothStatus::Disconnected => "bluetooth-disconnected",
            BluetoothStatus::Connected => "bluetooth-active",
        });
    }
}
