use bike_bt::{Address, Device, DeviceSignalStrength, DeviceStatus};
use relm4::{
    adw::{
        prelude::{ActionRowExt, PreferencesRowExt},
        ActionRow, Bin, PreferencesGroup,
    },
    gtk::{glib::clone, Label},
    prelude::{DynamicIndex, FactoryComponent},
};

pub struct DeviceListing {
    pub device: bike_bt::Device,
}

#[derive(Debug)]
pub enum DeviceListingOutput {
    Connect(Address, String),
}

impl FactoryComponent for DeviceListing {
    type ParentWidget = PreferencesGroup;
    type CommandOutput = ();
    type Input = ();
    type Output = DeviceListingOutput;
    type Init = Device;
    type Root = ActionRow;
    type Widgets = ();
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
        let suffix = Label::builder()
            .label(if self.device.signal == DeviceSignalStrength::NoSignal {
                "Not Available"
            } else {
                match self.device.status {
                    DeviceStatus::NotSetUp => "Not Set Up",
                    DeviceStatus::Paired => "Disconnected",
                    DeviceStatus::Connected => "Connected",
                }
            })
            .css_classes(["dim-label"])
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
    }

    fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _ender: relm4::FactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }
}
