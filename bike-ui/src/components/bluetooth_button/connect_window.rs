use bike_bt::{Address, Device};
use futures::StreamExt;
use relm4::{
    adw::{
        prelude::{AdwDialogExt, PreferencesPageExt},
        PreferencesGroup, PreferencesPage,
    },
    gtk::{glib::clone, prelude::WidgetExt},
    prelude::{AsyncComponentParts, FactoryVecDeque, SimpleAsyncComponent},
    spawn_local,
};

use crate::components::app::APP_DATA;

use super::DeviceListing;

pub struct ConnectDialog {
    #[allow(dead_code)]
    devices: FactoryVecDeque<DeviceListing>,
    join_handle: relm4::gtk::glib::JoinHandle<()>,
}

#[derive(Debug)]
pub enum ConnectDialogInput {
    StartScanning,
    StopScanning,
    DeviceAdded(Device),
    DeviceRemoved(Address),
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
        sender: relm4::AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        root.connect_realize(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(ConnectDialogInput::StartScanning);
            }
        ));
        root.connect_closed(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(ConnectDialogInput::StopScanning);
            }
        ));

        let preference_page = PreferencesPage::builder()
            .margin_end(20)
            .margin_start(20)
            .margin_bottom(20)
            .margin_top(20)
            .build();

        let preference_group = PreferencesGroup::builder()
            .title("Devices")
            .description("Please ensure your device is discoverable")
            .build();
        preference_page.add(&preference_group);

        let devices: FactoryVecDeque<DeviceListing> =
            FactoryVecDeque::builder().launch(preference_group).detach();

        root.set_child(Some(&preference_page));

        AsyncComponentParts {
            model: ConnectDialog {
                devices,
                join_handle: spawn_local(async {}),
            },
            widgets: (),
        }
    }
    async fn update(&mut self, message: Self::Input, sender: relm4::AsyncComponentSender<Self>) {
        match message {
            ConnectDialogInput::StartScanning => {
                self.devices.guard().clear();
                let join_handle = spawn_local(async move {
                    if let Some(bike_bt) = APP_DATA.read().bike_bt.as_ref() {
                        if let Ok(stream) = bike_bt.scan().await {
                            stream
                                .for_each(|e| async {
                                    let event = e;
                                    match event {
                                        bike_bt::DeviceDiscoveryEvent::DeviceAdded(device) => {
                                            sender.input(ConnectDialogInput::DeviceAdded(device));
                                        }
                                        bike_bt::DeviceDiscoveryEvent::DeviceRemoved(address) => {
                                            sender
                                                .input(ConnectDialogInput::DeviceRemoved(address));
                                        }
                                    }
                                })
                                .await;
                        }
                    }
                });
                self.join_handle = join_handle;
            }
            ConnectDialogInput::StopScanning => {
                self.join_handle.abort();
            }
            ConnectDialogInput::DeviceAdded(device) => {
                if !self
                    .devices
                    .iter()
                    .any(|listing| listing.device.address == device.address)
                {
                    self.devices.guard().push_back(device);
                }
            }
            ConnectDialogInput::DeviceRemoved(address) => {
                let index = self
                    .devices
                    .iter()
                    .position(|listing| listing.device.address == address.to_string());
                if let Some(index) = index {
                    self.devices.guard().remove(index);
                }
            }
        };
    }
}
