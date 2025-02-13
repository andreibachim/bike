mod bluetooth;
mod bluetooth_error;
mod bluetooth_status;
mod device;

pub use bluetooth::BikeBt;
pub use bluetooth_error::BluetoothError;
pub use bluetooth_status::BluetoothStatus;
pub use device::Device;
pub use device::DeviceDiscoveryEvent;

#[cfg(test)]
mod tests {
    use crate::{
        BluetoothStatus,
        DeviceDiscoveryEvent::{DeviceAdded, DeviceRemoved},
    };

    use super::BikeBt;
    use futures::StreamExt;

    #[tokio::test]
    async fn main_test() {
        let bike_bt = BikeBt::new().await;
        match bike_bt {
            Ok(bike_bt) => {
                assert!(bike_bt.get_status().await == BluetoothStatus::Disconnected);
                match bike_bt.scan().await {
                    Ok(device_change) => {
                        device_change
                            .for_each(|item| async move {
                                match item {
                                    DeviceAdded(bike_device) => {
                                        println!("Device found: {:#?}", bike_device)
                                    }
                                    DeviceRemoved(bike_device) => {
                                        println!("Device removed: {:#?}", bike_device)
                                    }
                                }
                            })
                            .await;
                    }
                    Err(_error) => eprintln!("Could not scan for devices. Sorry!"),
                }
            }
            Err(_error) => eprint!("Could not create bluetooth service."),
        };
    }
}
