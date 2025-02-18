use bike_bt::{BikeBt, ConnectedDevice};

#[derive(Default)]
pub struct AppData {
    pub bike_bt: Option<BikeBt>,
    pub device: Option<ConnectedDevice>,
}
