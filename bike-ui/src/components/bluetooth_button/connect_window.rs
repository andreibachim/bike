use bike_bt::Device;
use futures::StreamExt;
use relm4::{
    adw::{
        prelude::{AdwDialogExt, PreferencesPageExt},
        PreferencesGroup, PreferencesPage,
    },
    gtk::glib::MainContext,
    prelude::{AsyncComponentParts, AsyncFactoryVecDeque, SimpleAsyncComponent}, 
};

use crate::components::app::APP_DATA;

use super::DeviceListing;

pub struct ConnectDialog {
    #[allow(dead_code)]
    devices: AsyncFactoryVecDeque<DeviceListing>,
    join_handle: Option<relm4::gtk::glib::JoinHandle<()>>,
}

#[derive(Debug)]
pub enum ConnectDialogInput {
    StartScanning,
    StopScanning,
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
            model: ConnectDialog { devices, join_handle: None },
            widgets: (),
        }
    }
    async fn update(&mut self, message: Self::Input, sender: relm4::AsyncComponentSender<Self>) {
        match message {
            ConnectDialogInput::StartScanning => {
                self.devices.guard().clear();
                let join_handle = MainContext::default().spawn_local(async move {
                    if let Some(bike_bt) = APP_DATA.read().bike_bt.as_ref() {
                        if let Ok(stream) = bike_bt.scan().await {
                            stream
                                .for_each(|e| async {
                                    let event = e;
                                    match event {
                                        bike_bt::DeviceDiscoveryEvent::DeviceAdded(device) => {
                                            sender.input(ConnectDialogInput::DeviceAdded(device));
                                        }
                                        bike_bt::DeviceDiscoveryEvent::DeviceRemoved(device) => {
                                            sender.input(ConnectDialogInput::DeviceRemoved(device));
                                        }
                                    }
                                })
                                .await;
                        }
                    }
                });
                self.join_handle = Some(join_handle);
            }
            ConnectDialogInput::StopScanning => {
                self.join_handle.take().inspect(|handle| {
                    handle.abort();
                });
            }
            ConnectDialogInput::DeviceAdded(device) => {
                println!("Add a new device");
                self.devices.guard().push_back(device);
            }
            ConnectDialogInput::DeviceRemoved(device) => {
                println!("Remove a device");
                println!("{:#?}", device);
            }
        };
    }
}
