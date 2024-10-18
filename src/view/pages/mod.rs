use gtk::prelude::{ButtonExt, CastNone, GtkWindowExt, StaticType, WidgetExt};

pub fn routes_page() -> adw::NavigationPage {
    let toolbar_view = adw::ToolbarView::builder().build();
    toolbar_view.add_top_bar(&header_bar());
    let navigation_page = adw::NavigationPage::builder()
        .child(&toolbar_view)
        .title("Routes")
        .build();
    navigation_page
}

fn header_bar() -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::builder().build();
    header_bar.pack_end(&devices());
    header_bar
}

fn devices() -> gtk::Button {
    let button = gtk::Button::builder()
        .css_classes(["error"])
        .icon_name("bluetooth-disabled")
        .build();
    button.connect_clicked(|button| {
        let main_window: adw::ApplicationWindow = button
            .ancestor(gtk::Window::static_type())
            .and_downcast()
            .unwrap();
        let device_window = adw::Window::builder()
            .transient_for(&main_window)
            .modal(true)
            .destroy_with_parent(true)
            .build();
        device_window.present();
    });
    button
}
