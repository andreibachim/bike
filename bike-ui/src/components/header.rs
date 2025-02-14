use std::rc::Rc;

use bike_bt::BikeBt;
use relm4::prelude::{
    AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
    SimpleAsyncComponent,
};

use super::bluetooth_button::BluetoothButton;

pub struct Header {
    bluetooth_button: AsyncController<BluetoothButton>,
}

impl SimpleAsyncComponent for Header {
    type Input = ();
    type Output = ();
    type Init = Rc<Option<BikeBt>>;
    type Root = relm4::adw::HeaderBar;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::adw::HeaderBar::new()
    }

    async fn init(
        bike_bt: Self::Init,
        root: Self::Root,
        _sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let bluetooth_button = BluetoothButton::builder().launch(bike_bt).detach();
        root.pack_end(bluetooth_button.widget());
        let model = Header { bluetooth_button };
        AsyncComponentParts { model, widgets: () }
    }
}
