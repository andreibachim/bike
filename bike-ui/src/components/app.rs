use bike_bt::{BikeBt, BluetoothStatus};
use relm4::{
    adw::prelude::AdwApplicationWindowExt,
    prelude::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, SimpleAsyncComponent,
    },
};

use crate::components::bluetooth_button::BLUETOOTH_STATUS_BROKER;

use super::{bluetooth_button::BluetoothButtonInput, Header};

pub struct App;

impl SimpleAsyncComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = relm4::adw::ApplicationWindow;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::adw::ApplicationWindow::builder()
            .title("Bike")
            .default_width(1000)
            .default_height(900)
            .build()
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        match BikeBt::new().await {
            Ok(bike_bt) => {
                BLUETOOTH_STATUS_BROKER
                    .send(BluetoothButtonInput::SetStatus(bike_bt.get_status().await));
                Some(bike_bt)
            }
            Err(error) => {
                BLUETOOTH_STATUS_BROKER.send(BluetoothButtonInput::SetStatus(
                    BluetoothStatus::Unavailable,
                ));
                eprintln!("Could not start bluetooth session. Error: {}", error);
                None
            }
        };

        //Create the header
        let header = Header::builder().launch(()).detach();

        //Setup the toolbar view
        let toolbar_view = relm4::adw::ToolbarView::new();
        toolbar_view.add_top_bar(header.widget());
        root.set_content(Some(&toolbar_view));

        AsyncComponentParts {
            model: App {},
            widgets: (),
        }
    }
}
