use std::time::Duration;

use bike_bt::{Address, Device};
use relm4::{
    adw::{
        prelude::{AdwDialogExt, PreferencesPageExt},
        HeaderBar, NavigationPage, NavigationView, PreferencesGroup, PreferencesPage, ToolbarView,
    },
    gtk::{
        glib::{clone, timeout_future},
        prelude::WidgetExt,
    },
    prelude::{AsyncComponentParts, FactoryVecDeque, SimpleAsyncComponent},
    Component, ComponentController, Controller,
};

use crate::{
    brokers::STATE_MANAGER, components::active_device_details::{ActiveDeviceDetails, ACTIVE_DEVICE_DETAILS_BROKER},
    state_manager::StateManagerInput,
};

use super::{
    device_discovery_listener::{DeviceDiscoveryListener, DEVICE_DISCOVER_BROKER},
    device_listing::{DeviceListing, DeviceListingOutput},
};

pub struct ConnectDialog {
    #[allow(dead_code)]
    devices: FactoryVecDeque<DeviceListing>,
    #[allow(dead_code)]
    device_discovery_listener: Controller<DeviceDiscoveryListener>,
    navigation_view: NavigationView,
    #[allow(dead_code)]
    active_device_details: Controller<ActiveDeviceDetails>,
}

#[derive(Debug)]
pub enum ConnectDialogInput {
    StartScanning,
    StopScanning,
    DeviceAdded(Device),
    DeviceRemoved(Address),
    Connect(Address, String),
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
            move |_| sender.input(ConnectDialogInput::StartScanning)
        ));

        root.connect_closed(clone!(
            #[strong]
            sender,
            move |_| sender.input(ConnectDialogInput::StopScanning)
        ));

        let device_discovery_listener = DeviceDiscoveryListener::builder()
            .launch_with_broker((), &DEVICE_DISCOVER_BROKER)
            .forward(sender.input_sender(), |msg| match msg {
                super::device_discovery_listener::DeviceDiscoveryEvent::DeviceFound(device) => {
                    ConnectDialogInput::DeviceAdded(device)
                }
                super::device_discovery_listener::DeviceDiscoveryEvent::DeviceLost(address) => {
                    ConnectDialogInput::DeviceRemoved(address)
                }
            });

        let navigation_view = NavigationView::builder().pop_on_escape(false).build();
        root.set_child(Some(&navigation_view));

        let (scan_page, preferences_group) = create_scan_page();
        navigation_view.add(&scan_page);

        let devices: FactoryVecDeque<DeviceListing> = FactoryVecDeque::builder()
            .launch(preferences_group)
            .forward(sender.input_sender(), |message| match message {
                DeviceListingOutput::Connect(address, name) => ConnectDialogInput::Connect(address, name),
            });

        let active_device_details = ActiveDeviceDetails::builder().launch_with_broker((), &ACTIVE_DEVICE_DETAILS_BROKER).detach();

        navigation_view.add(active_device_details.widget());

        AsyncComponentParts {
            model: ConnectDialog {
                devices,
                device_discovery_listener,
                active_device_details,
                navigation_view,
            },
            widgets: (),
        }
    }
    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match message {
            ConnectDialogInput::StartScanning => {
                self.devices.guard().clear();
                crate::brokers::STATE_MANAGER.send(StateManagerInput::StartScanningForDevices);
            }
            ConnectDialogInput::StopScanning => {
                crate::brokers::STATE_MANAGER.send(StateManagerInput::StopScanningForDevices);
                timeout_future(Duration::from_millis(200)).await;
                self.navigation_view.pop_to_tag("scan");
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
                    .position(|listing| listing.device.address == address);
                if let Some(index) = index {
                    self.devices.guard().remove(index);
                }
            }
            ConnectDialogInput::Connect(address, name) => {
                self.navigation_view.push_by_tag("connect");
                STATE_MANAGER.send(StateManagerInput::Connect(address, name));
            }
        };
    }
}

fn create_scan_page() -> (NavigationPage, PreferencesGroup) {
    let container = ToolbarView::builder().build();
    container.add_top_bar(&HeaderBar::new());

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
    container.set_content(Some(&preference_page));

    (
        NavigationPage::builder()
            .title("Select device")
            .tag("scan")
            .child(&container)
            .build(),
        preference_group,
    )
}
