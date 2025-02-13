use bike_bt::BluetoothStatus;
use relm4::{
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    Component, ComponentController, Controller,
};

use super::{bluetooth_button::BluetoothButton, bluetooth_button::BLUETOOTH_STATUS_BROKER};

pub struct Header {
    bluetooth_button: Controller<BluetoothButton>,
}

impl SimpleAsyncComponent for Header {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = relm4::adw::HeaderBar;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::adw::HeaderBar::new()
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let bluetooth_button = BluetoothButton::builder()
            .launch_with_broker(BluetoothStatus::Unavailable, &BLUETOOTH_STATUS_BROKER)
            .detach();
        root.pack_start(bluetooth_button.widget());
        let model = Header { bluetooth_button };
        AsyncComponentParts { model, widgets: () }
    }
}
