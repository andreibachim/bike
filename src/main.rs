mod components;

use adw::prelude::AdwApplicationWindowExt;
use gtk::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
};

const APP_ID: &str = "io.github.andreibachim.bike";

fn main() -> ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Bike")
        .default_width(1000)
        .default_height(700)
        .build();
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&crate::components::header());
    toolbar_view.set_content(Some(&gtk::Label::builder().label("Hello, world").build()));
    window.set_content(Some(&toolbar_view));
    window.present();
}
