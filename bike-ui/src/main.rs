use crate::components::App;
use relm4::RelmApp;
mod components;

const APPLICATION_ID: &str = "io.github.andreibachim.bike";

fn main() {
    let app = RelmApp::new(APPLICATION_ID);
    app.run_async::<App>(());
}
