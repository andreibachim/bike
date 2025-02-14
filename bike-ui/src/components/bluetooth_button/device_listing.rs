use bike_bt::Device;
use relm4::{
    adw::{ActionRow, PreferencesGroup},
    gtk::prelude::{BoxExt, ListBoxRowExt, WidgetExt},
    prelude::AsyncFactoryComponent,
};

pub struct DeviceListing {
    device: bike_bt::Device,
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
        ActionRow::new()
    }

    fn init_widgets(
        &mut self,
        _index: &relm4::prelude::DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: relm4::AsyncFactorySender<Self>,
    ) -> Self::Widgets {
        let container = relm4::gtk::Box::builder()
            .orientation(relm4::gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
        container.set_css_classes(&["header"]);
        let device_name = relm4::gtk::Label::builder()
            .label(&self.device.name)
            .hexpand(true)
            .halign(relm4::gtk::Align::Start)
            .build();
        container.append(&device_name);
        root.set_child(Some(&container));
        println!("Hello");
    }

    async fn init_model(
        device: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _sender: relm4::AsyncFactorySender<Self>,
    ) -> Self {
        DeviceListing { device }
    }
}
