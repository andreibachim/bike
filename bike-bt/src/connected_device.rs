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
        if let Some(services) = self.device.uuids().await.map_err(|e| {
            eprintln!("Could not detect UUIDs. Error: {e}");
            ()
        })? {
            for service in services {
                println!("Service found: {}", service);
            }
        };
        Ok(())
    }
}
