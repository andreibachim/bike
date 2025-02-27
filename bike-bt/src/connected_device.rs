#[derive(Debug)]
pub struct ConnectedDevice {
    pub device: bluer::Device,
    pub name: String,
}

impl ConnectedDevice {
    pub fn new(device: bluer::Device, name: String) -> Self {
        Self { device, name }
    }

    pub async fn disconnect(&self) -> Result<(), ()> {
        if self.device.is_connected().await.map_err(|_| ())? {
            self.device.disconnect().await.map_err(|_| ())
        } else {
            Ok(())
        }
    }

    pub async fn get_gatt_services(&self) -> Result<(), ()> {
        for service in self.device.services().await.map_err(|e| {
            eprintln!("Could not detect GATT services. Error: {e}");
            ()
        })? {
            println!("{:#?}", service.uuid().await);
        }
        Ok(())
    }
}
