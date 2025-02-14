use std::rc::Rc;

use bike_bt::{BikeBt, BluetoothStatus};
use futures::StreamExt;
use relm4::{
    adw::prelude::AdwDialogExt,
    gtk::{
        glib::{clone, MainContext},
        prelude::{ButtonExt, WidgetExt},
    },
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    Component, ComponentController, Controller, RelmWidgetExt,
};

use super::ConnectDialog;

pub struct BluetoothButton {
    status: BluetoothStatus,
    bike_bt: Rc<Option<BikeBt>>,
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

impl SimpleAsyncComponent for BluetoothButton {
    type Input = BluetoothButtonInput;
    type Output = ();
    type Init = Rc<Option<BikeBt>>;
    type Root = relm4::gtk::Button;
    type Widgets = BluetoothStatusWidgets;

    fn init_root() -> Self::Root {
        relm4::gtk::Button::builder()
            .focusable(false)
            .icon_name("bluetooth-hardware-disabled")
            .build()
    }

    async fn init(
        bike_bt: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        if let Some(bike_bt) = bike_bt.as_ref() {
            sender.input(BluetoothButtonInput::SetStatus(bike_bt.get_status().await));
            if let Ok(event_stream) = bike_bt.register_adapter_listener().await {
                let sender = sender.clone();
                MainContext::default().spawn_local(async move {
                    Box::pin(event_stream.for_each(|item| async {
                        sender.input(BluetoothButtonInput::SetStatus(item));
                    }))
                    .await;
                });
            }
        };

        let connect_dialog = ConnectDialog::builder().launch(root.clone()).detach();

        root.connect_clicked(clone!(move |_| sender.input(BluetoothButtonInput::Clicked)));
        let model = BluetoothButton {
            status: BluetoothStatus::Connected,
            bike_bt,
            connect_dialog,
        };

        let widgets = BluetoothStatusWidgets { button: root };
        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
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

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::AsyncComponentSender<Self>) {
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
