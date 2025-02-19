use relm4::{
    adw::{prelude::AdwApplicationWindowExt, HeaderBar, ViewStack, ViewSwitcher},
    gtk::{prelude::BoxExt, Label},
    prelude::{AsyncComponent, AsyncController},
    Component, ComponentController, ComponentParts, Controller, SimpleComponent,
};

use crate::{brokers::STATE_MANAGER, state_manager::StateManager};

use crate::components::bluetooth_button::button::{BluetoothButton, ADAPTER_STATE_BROKER};

pub struct App {
    #[allow(dead_code)]
    state_manager: AsyncController<StateManager>,
    #[allow(dead_code)]
    bluetooth_button: Controller<BluetoothButton>,
}

impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = relm4::adw::ApplicationWindow;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::adw::ApplicationWindow::builder()
            .title("Bike")
            .default_width(1000)
            .default_height(900)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let state_manager = StateManager::builder()
            .launch_with_broker((), &STATE_MANAGER)
            .detach();

        //Create the header
        let header_bar = HeaderBar::new();
        let bluetooth_button = BluetoothButton::builder()
            .launch_with_broker((), &ADAPTER_STATE_BROKER)
            .detach();
        header_bar.pack_end(bluetooth_button.widget());

        //Setup the toolbar view
        let toolbar_view = relm4::adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header_bar);

        let content = relm4::gtk::Box::new(relm4::gtk::Orientation::Vertical, 10);
        let view_stack = ViewStack::builder().build();
        let view_switcher = ViewSwitcher::builder()
            .halign(relm4::gtk::Align::Center)
            .hexpand(false)
            .policy(relm4::adw::ViewSwitcherPolicy::Wide)
            .stack(&view_stack)
            .build();
        header_bar.set_title_widget(Some(&view_switcher));

        //Add a couple of pages
        view_stack.add_titled_with_icon(
            &Label::new(Some("")),
            Some("routes"),
            "Routes",
            "org.gnome.Maps-symbolic",
        );
        view_stack.add_titled_with_icon(
            &Label::new(Some("")),
            Some("history"),
            "History",
            "accessories-text-editor",
        );

        content.append(&view_stack);

        toolbar_view.set_content(Some(&content));

        //Content
        root.set_content(Some(&toolbar_view));

        ComponentParts {
            model: App {
                state_manager,
                bluetooth_button,
            },
            widgets: (),
        }
    }
}
