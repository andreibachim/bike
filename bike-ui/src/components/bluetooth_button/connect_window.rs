use relm4::{ComponentParts, SimpleComponent};

pub struct ConnectDialog {
    pub owner: relm4::gtk::Button,
}

impl SimpleComponent for ConnectDialog {
    type Input = ();
    type Output = ();
    type Init = relm4::gtk::Button;
    type Root = relm4::adw::Dialog;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::adw::Dialog::builder()
            .presentation_mode(relm4::adw::DialogPresentationMode::Floating)
            .content_width(600)
            .content_height(500)
            .can_close(true)
            .build()
    }

    fn init(
        owner: Self::Init,
        _root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        ComponentParts {
            model: ConnectDialog { owner },
            widgets: (),
        }
    }
}
