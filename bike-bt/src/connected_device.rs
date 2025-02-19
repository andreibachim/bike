#[derive(Debug)]
pub struct ConnectedDevice {
    device: bluer::Device,
}

impl ConnectedDevice {
    pub fn new(device: bluer::Device) -> Self {
        Self { device }
    }
}
