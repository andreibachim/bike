use bike_bt::Device;
use relm4::{
    adw::{
        prelude::{ActionRowExt, PreferencesRowExt},
        ActionRow, PreferencesGroup,
    },
    gtk::Label,
    prelude::{DynamicIndex, FactoryComponent},
};

pub struct DeviceListing {
    pub device: bike_bt::Device,
}

impl FactoryComponent for DeviceListing {
    type ParentWidget = PreferencesGroup;
    type CommandOutput = ();
    type Input = ();
    type Output = ();
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
        _sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        root.set_title(&self.device.name);

        let suffix = Label::builder()
            .label(match self.device.paired {
                true => "Disconnected",
                false => "Not Set Up",
            })
            .css_classes(["dim-label"])
            .build();
        root.set_activatable_widget(Some(&suffix));
        root.add_suffix(&suffix);

        root.connect_activated(|_| {
            println!("Hello");
        });
    }

    fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }
}
