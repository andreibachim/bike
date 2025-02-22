use crate::components::App;
use relm4::RelmApp;
mod components;
mod brokers;
mod state_manager;

const APPLICATION_ID: &str = "io.github.andreibachim.bike";

fn main() {
    let app = RelmApp::new(APPLICATION_ID);
    initialize_custom_icons();  
    app.run::<App>(());
}

fn initialize_custom_icons() {
    relm4::gtk::gio::resources_register_include!("resources.gresource").expect("Could not register custom resources");
    let display = relm4::gtk::gdk::Display::default().expect("There is no display available");
    let theme = relm4::gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/io/github/andreibachim/bike/icons");
}

