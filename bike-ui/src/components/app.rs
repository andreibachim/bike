use std::rc::Rc;

use bike_bt::BikeBt;
use relm4::{
    adw::prelude::AdwApplicationWindowExt,
    prelude::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
};

use super::Header;

pub struct App {
    header: AsyncController<Header>,
    bike_bt: Rc<Option<BikeBt>>,
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

        let bike_bt = Rc::new(bike_bt);

        //Create the header
        let header = Header::builder().launch(bike_bt.clone()).detach();

        //Setup the toolbar view
        let toolbar_view = relm4::adw::ToolbarView::new();
        toolbar_view.add_top_bar(header.widget());
        root.set_content(Some(&toolbar_view));

        AsyncComponentParts {
            model: App { header, bike_bt },
            widgets: (),
        }
    }
}
