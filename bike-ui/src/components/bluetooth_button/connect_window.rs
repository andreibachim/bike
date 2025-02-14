use bike_bt::Device;
use relm4::{
    adw::{
        prelude::{AdwDialogExt, PreferencesPageExt},
        PreferencesGroup, PreferencesPage,
    },
    prelude::{AsyncComponentParts, AsyncFactoryVecDeque, SimpleAsyncComponent},
};


use super::DeviceListing;

pub struct ConnectDialog {
    #[allow(dead_code)]
    devices: AsyncFactoryVecDeque<DeviceListing>,
}

#[derive(Debug)]
pub enum ConnectDialogInput {
    StartScanning,
    DeviceAdded(Device),
    DeviceRemoved(Device),
}

impl SimpleAsyncComponent for ConnectDialog {
    type Input = ConnectDialogInput;
    type Output = ();
    type Init = ();
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

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let preference_page = PreferencesPage::builder()
            .margin_end(20)
            .margin_start(20)
            .margin_bottom(20)
            .margin_top(20)
            .build();

        let preference_group = PreferencesGroup::builder()
            .title("Devices")
            .description("Available devices")
            .build();
        preference_page.add(&preference_group);

        let devices: AsyncFactoryVecDeque<DeviceListing> = AsyncFactoryVecDeque::builder()
            .launch(preference_group)
            .detach();

        root.set_child(Some(&preference_page));

        AsyncComponentParts {
            model: ConnectDialog { devices },
            widgets: (),
        }
    }

    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match message {
            ConnectDialogInput::StartScanning => {}
            ConnectDialogInput::DeviceAdded(_device) => todo!(),
            ConnectDialogInput::DeviceRemoved(_device) => todo!(),
        };
    }
}
