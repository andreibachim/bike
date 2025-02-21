#[derive(Debug)]
pub struct ConnectedDevice {
    pub device: bluer::Device,
    pub name: String,
}

impl ConnectedDevice {
    pub fn new(device: bluer::Device, name: String) -> Self {
        Self { 
            device,
            name
        }
    }
}
