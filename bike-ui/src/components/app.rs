use bike_bt::BikeBt;
use relm4::{
    adw::{prelude::AdwApplicationWindowExt, HeaderBar},
    prelude::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
    SharedState,
};

use crate::app_data::AppData;

use super::bluetooth_button::BluetoothButton;

pub static APP_DATA: SharedState<AppData> = SharedState::new();

pub struct App {
    #[allow(dead_code)]
    bluetooth_button: AsyncController<BluetoothButton>,
}

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
        let bike_bt = match BikeBt::new().await {
            Ok(bike_bt) => Some(bike_bt),
            Err(error) => {
                eprintln!("Could not start bluetooth session. Error: {}", error);
                todo!("Show error dialog");
            }
        };

        APP_DATA.write_inner().bike_bt = bike_bt;

        //Create the header
        let header_bar = HeaderBar::new();
        let bluetooth_button = BluetoothButton::builder().launch(()).detach();
        header_bar.pack_end(bluetooth_button.widget());

        //Setup the toolbar view
        let toolbar_view = relm4::adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header_bar);
        root.set_content(Some(&toolbar_view));

        AsyncComponentParts {
            model: App { bluetooth_button },
            widgets: (),
        }
    }
}
