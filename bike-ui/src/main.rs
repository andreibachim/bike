use crate::components::App;
use relm4::RelmApp;
mod components;
mod brokers;
mod state_manager;

const APPLICATION_ID: &str = "io.github.andreibachim.bike";

fn main() {
    let app = RelmApp::new(APPLICATION_ID);
    app.run::<App>(());
}
