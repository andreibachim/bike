use relm4::{
    adw::{
        prelude::{ActionRowExt, NavigationPageExt, PreferencesGroupExt},
        ActionRow, HeaderBar, NavigationPage, PreferencesGroup, Spinner, ToolbarView,
    },
    gtk::{
        glib::clone,
        prelude::{ButtonExt, WidgetExt},
        Button, CenterBox, Image,
    },
    Component, ComponentParts, MessageBroker,
};

use crate::{brokers::STATE_MANAGER, state_manager::StateManagerInput};

pub struct ActiveDeviceDetails {
    name: Option<String>,
    connected: bool,
}

pub static ACTIVE_DEVICE_DETAILS_BROKER: MessageBroker<ActiveDeviceDetailsInput> =
    MessageBroker::new();

#[derive(Debug)]
pub enum ActiveDeviceDetailsInput {
    SetName(String),
    SetConnected,
    ConnectionFailed,
}

#[derive(Debug)]
pub enum ActiveDeviceDetailsOutput {
    CloseDialog,
    GoBack,
}

pub struct ActiveDeviceDetailsWidgets {
    root: NavigationPage,
    preferences_group: PreferencesGroup,
    disconnect_button: Button,
    ride_button: Button,
}

impl Component for ActiveDeviceDetails {
    type CommandOutput = ();
    type Input = ActiveDeviceDetailsInput;
    type Output = ActiveDeviceDetailsOutput;
    type Init = ();
    type Root = NavigationPage;
    type Widgets = ActiveDeviceDetailsWidgets;

    fn init_root() -> Self::Root {
        NavigationPage::builder()
            .title("Device details")
            .can_pop(false)
            .tag("connect")
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let container = ToolbarView::builder().build();
        container.add_top_bar(&HeaderBar::builder().show_back_button(false).build());
        //Create the preferences group
        let preferences_group = PreferencesGroup::builder().build();
        preferences_group.set_title("DEVICE CAPABILITIES");
        preferences_group.set_description(Some("Connecting..."));
        preferences_group.set_header_suffix(Some(&Spinner::builder().build()));

        //Control capability
        let control = ActionRow::builder()
            .title("Control")
            .subtitle("Can control device resistance")
            .build();
        control.add_suffix(&Spinner::builder().build());
        preferences_group.add(&control);

        //Power capability
        let power = ActionRow::builder()
            .title("Power")
            .subtitle("Can track power output")
            .build();
        power.add_suffix(&Spinner::builder().build());
        preferences_group.add(&power);

        //Cadence capability
        let cadence = ActionRow::builder()
            .title("Cadence")
            .subtitle("Provides cadence measurements")
            .build();
        cadence.add_suffix(&Spinner::builder().build());
        preferences_group.add(&cadence);

        let center_container = CenterBox::builder()
            .orientation(relm4::gtk::Orientation::Vertical)
            .margin_end(50)
            .margin_start(50)
            .margin_top(0)
            .margin_bottom(50)
            .build();
        center_container.set_center_widget(Some(&preferences_group));
        container.set_content(Some(&center_container));

        root.set_child(Some(&container));

        //Set control buttons
        let controls = CenterBox::builder().build();

        let disconnect_button = Button::builder()
            .css_classes(["destructive-action"])
            .width_request(100)
            .label("Cancel")
            .build();
        disconnect_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.output(ActiveDeviceDetailsOutput::GoBack).unwrap();
                crate::brokers::STATE_MANAGER.send(StateManagerInput::Disconnect);
            }
        ));

        controls.set_start_widget(Some(&disconnect_button));

        let ride_button = Button::builder()
            .sensitive(false)
            .width_request(100)
            .css_classes(["suggested-action"])
            .label("Ok")
            .build();
        ride_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender
                    .output(ActiveDeviceDetailsOutput::CloseDialog)
                    .unwrap();
            }
        ));
        controls.set_end_widget(Some(&ride_button));

        center_container.set_end_widget(Some(&controls));

        let widgets = ActiveDeviceDetailsWidgets {
            root,
            preferences_group,
            disconnect_button,
            ride_button,
        };

        ComponentParts {
            model: Self {
                name: None,
                connected: false,
            },
            widgets,
        }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ActiveDeviceDetailsInput::SetName(name) => self.name = Some(name),
            ActiveDeviceDetailsInput::SetConnected => {
                self.connected = true;
                STATE_MANAGER.send(StateManagerInput::DiscoverGattProfiles);
            }
            ActiveDeviceDetailsInput::ConnectionFailed => {
                self.connected = false;
                sender
                    .output(ActiveDeviceDetailsOutput::GoBack)
                    .expect("Could not go back")
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::ComponentSender<Self>) {
        widgets.root.set_title(
            &self
                .name
                .clone()
                .unwrap_or("Device placeholder name".to_string()),
        );
        if self.connected {
            widgets.preferences_group.set_description(Some("Connected"));
            widgets.preferences_group.set_header_suffix(Some(
                &Image::builder()
                    .icon_size(relm4::gtk::IconSize::Large)
                    .css_classes(["success"])
                    .icon_name("checkmark-symbolic")
                    .build(),
            ));
            widgets.disconnect_button.set_label("Disconnect");
            widgets.ride_button.set_sensitive(true);
        }
    }
}
