use bike_bt::BluetoothStatus;
use futures::StreamExt;
use relm4::{
    adw::prelude::AdwDialogExt,
    gtk::{
        glib::{clone, MainContext},
        prelude::{ButtonExt, WidgetExt},
    },
    prelude::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
    RelmWidgetExt,
};

use crate::components::app::APP_DATA;

use super::{connect_window::ConnectDialogInput, ConnectDialog};

pub struct BluetoothButton {
    status: BluetoothStatus,
    connect_dialog: AsyncController<ConnectDialog>,
}

#[derive(Debug)]
pub enum BluetoothButtonInput {
    SetStatus(BluetoothStatus),
    Clicked(relm4::gtk::Button),
}

pub struct BluetoothStatusWidgets {
    button: relm4::gtk::Button,
}

impl SimpleAsyncComponent for BluetoothButton {
    type Input = BluetoothButtonInput;
    type Output = ();
    type Init = ();
    type Root = relm4::gtk::Button;
    type Widgets = BluetoothStatusWidgets;

    fn init_root() -> Self::Root {
        relm4::gtk::Button::builder()
            .focusable(false)
            .icon_name("bluetooth-hardware-disabled")
            .build()
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        if let Some(bike_bt) = APP_DATA.read().bike_bt.as_ref() {
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

        let connect_dialog = ConnectDialog::builder().launch(()).detach();

        root.connect_clicked(clone!(
            move |btn| sender.input(BluetoothButtonInput::Clicked(btn.clone()))
        ));
        let model = BluetoothButton {
            status: BluetoothStatus::Connected,
            connect_dialog,
        };

        let widgets = BluetoothStatusWidgets { button: root };
        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match message {
            BluetoothButtonInput::SetStatus(bluetooth_status) => self.status = bluetooth_status,
            BluetoothButtonInput::Clicked(owner) => match self.status {
                BluetoothStatus::Disconnected => {
                    if self
                        .connect_dialog
                        .sender()
                        .send(ConnectDialogInput::StartScanning)
                        .is_ok()
                    {
                        let widget = self.connect_dialog.widget();
                        let window = owner.toplevel_window();
                        widget.present(window.as_ref());
                    }
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
