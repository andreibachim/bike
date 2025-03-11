use bike_bt::{Address, Device, DeviceSignalStrength, DeviceStatus};
use relm4::{
    adw::{
        prelude::{ActionRowExt, PreferencesRowExt},
        ActionRow, Bin, PreferencesGroup,
    },
    gtk::{glib::clone, Image},
    prelude::{DynamicIndex, FactoryComponent},
};

pub struct DeviceListing {
    pub device: bike_bt::Device,
}

#[derive(Debug)]
pub enum DeviceListingOutput {
    Connect(Address, String),
}

pub struct DeviceListingWidgets {
    signal_icon: Image,
}

impl FactoryComponent for DeviceListing {
    type ParentWidget = PreferencesGroup;
    type CommandOutput = ();
    type Input = ();
    type Output = DeviceListingOutput;
    type Init = Device;
    type Root = ActionRow;
    type Widgets = DeviceListingWidgets;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        ActionRow::builder().build()
    }

    fn init_widgets(
        &mut self,
        _index: &relm4::prelude::DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        root.add_prefix(
            &relm4::gtk::Image::builder()
                .icon_name("checkmark-symbolic")
                .build(),
        );

        root.set_activatable_widget(Some(&Bin::new()));
        root.set_title(&self.device.name);
        root.set_subtitle(match self.device.status {
            DeviceStatus::NotSetUp => "Not Set Up",
            DeviceStatus::Paired => "Disconnected",
            DeviceStatus::Connected => "Connected",
        });
        let suffix = relm4::gtk::Image::builder()
            .icon_name(match self.device.signal {
                DeviceSignalStrength::NoSignal => "network-cellular-offline-symbolic",
                DeviceSignalStrength::Weak => "network-cellular-signal-weak-symbolic",
                DeviceSignalStrength::Ok => "network-cellular-signal-ok-symbolic",
                DeviceSignalStrength::Good => "network-cellular-signal-good-symbolic",
                DeviceSignalStrength::Full => "network-cellular-signal-excellent-symbolic",
            })
            .build();
        root.add_suffix(&suffix);
        root.connect_activated(clone!(
            #[strong]
            sender,
            #[strong(rename_to=address)]
            self.device.address,
            #[strong(rename_to=name)]
            self.device.name,
            move |_| {
                let _ = sender.output(DeviceListingOutput::Connect(address, name.clone()));
            }
        ));

        DeviceListingWidgets {
            signal_icon: suffix,
        }
    }

    fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _ender: relm4::FactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::FactorySender<Self>) {
        widgets.signal_icon.set_icon_name(match self.device.signal {
            DeviceSignalStrength::NoSignal => Some("network-cellular-offline-symbolic"),
            DeviceSignalStrength::Weak => Some("network-cellular-signal-weak-symbolic"),
            DeviceSignalStrength::Ok => Some("network-cellular-signal-ok-symbolic"),
            DeviceSignalStrength::Good => Some("network-cellular-signal-good-symbolic"),
            DeviceSignalStrength::Full => Some("network-cellular-signal-excellent-symbolic"),
        });
    }
}
