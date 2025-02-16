use bike_bt::Device;
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
    Connect,
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
        root.set_activatable_widget(Some(&Bin::new()));
        root.set_title(&self.device.name);
        let suffix = Label::builder()
            .label(match self.device.paired {
                true => "Disconnected",
                false => "Not Set Up",
            })
            .css_classes(["dim-label"])
            .build();
        root.add_suffix(&suffix);
        root.connect_activated(clone!(
            #[strong]
            sender,
            move |_| {
                sender.output(DeviceListingOutput::Connect).expect("Asd");
            }
        ));
    }

    fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }
}
