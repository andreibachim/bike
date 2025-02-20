mod bluetooth;
mod bluetooth_error;
mod bluetooth_status;
mod device;
mod connected_device;

pub use bluetooth::BikeBt;
pub use bluetooth_error::BluetoothError;
pub use bluetooth_status::BluetoothStatus;
pub use device::Device;
pub use device::DeviceStatus;
pub use device::DeviceDiscoveryEvent;
pub use device::DeviceSignalStrength;
pub use bluer::Address;
pub use connected_device::ConnectedDevice;


#[cfg(test)]
mod tests {
    use super::BikeBt;
    use futures::StreamExt;

    #[tokio::test]
    async fn main_test() {
        let bike_bt = BikeBt::new().await;
        match bike_bt {
            Ok(bike_bt) => {
                match bike_bt.register_adapter_listener().await {
                    Ok(a) => {
                        a.for_each(|adapter_property| async move {
                            println!("{:#?}", adapter_property);
                        })
                        .await;
                    }
                    Err(_) => {}
                }
                //match bike_bt.scan().await {
                //    Ok(device_change) => {
                //        device_change
                //            .for_each(|item| async move {
                //                match item {
                //                    DeviceAdded(bike_device) => {
                //                        println!("Device found: {:#?}", bike_device)
                //                    }
                //                    DeviceRemoved(bike_device) => {
                //                        println!("Device removed: {:#?}", bike_device)
                //                    }
                //                }
                //            })
                //            .await;
                //    }
                //    Err(_error) => eprintln!("Could not scan for devices. Sorry!"),
                //}
            }
            Err(_error) => eprint!("Could not create bluetooth service."),
        };
    }
}
