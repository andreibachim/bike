mod view;
mod singletons;

use adw::Application;
use gtk::gio::resources_register_include;
use gtk::glib;
use gtk::prelude::*;
use view::navigation_view;

const APP_ID: &str = "io.github.andreibachim.bike";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    resources_register_include!("bike.gresource").expect("Could not load resources");
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(600)
        .content(&navigation_view())
        .build();
    window.present();
}
