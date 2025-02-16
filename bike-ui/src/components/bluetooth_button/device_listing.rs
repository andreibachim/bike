use bike_bt::Device;
use relm4::{
    adw::{
        prelude::{ActionRowExt, PreferencesRowExt},
        ActionRow, PreferencesGroup,
    },
    prelude::AsyncFactoryComponent,
};

pub struct DeviceListing {
    pub device: bike_bt::Device,
}

impl AsyncFactoryComponent for DeviceListing {
    type ParentWidget = PreferencesGroup;
    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = Device;
    type Root = ActionRow;
    type Widgets = ();

    fn init_root() -> Self::Root {
        ActionRow::builder().build()
    }

    fn init_widgets(
        &mut self,
        _index: &relm4::prelude::DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: relm4::AsyncFactorySender<Self>,
    ) -> Self::Widgets {
        //Signal strength
        let signal_icon = relm4::gtk::Image::builder()
            .icon_name(match self.device.signal {
                bike_bt::DeviceSignalStrength::NoSignal => "network-cellular-offline-symbolic",
                bike_bt::DeviceSignalStrength::Weak => "network-cellular-signal-weak-symbolic",
                bike_bt::DeviceSignalStrength::Ok => "network-cellular-signal-ok-symbolic",
                bike_bt::DeviceSignalStrength::Good => "network-cellular-signal-good-symbolic",
                bike_bt::DeviceSignalStrength::Full => "network-cellular-signal-excellent-symbolic",
            })
            .build();
        root.add_suffix(&signal_icon);
        root.set_activatable_widget(Some(&signal_icon));

        root.set_title(&self.device.name);
        root.set_subtitle(match self.device.paired {
            true => "Disconnected",
            false => "Not Set Up",
        });

        root.connect_activated(|_| {
            println!("Hello");
        });
    }

    async fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _sender: relm4::AsyncFactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }
}
