use std::sync::Arc;

use crate::{BluetoothError, BluetoothStatus, Device, DeviceDiscoveryEvent};
use bluer::{
    Adapter,
    AdapterEvent::{DeviceAdded, DeviceRemoved, PropertyChanged},
    Address,
};
use futures::{stream::StreamExt, Stream};

pub struct BikeBt {
    runtime: tokio::runtime::Runtime,
    pub adapter: Arc<bluer::Adapter>,
}

impl BikeBt {
    pub async fn new() -> Result<Self, BluetoothError> {
        let runtime = tokio::runtime::Runtime::new().map_err(|_| BluetoothError::NoBluez)?;
        let adapter = runtime
            .spawn(async {
                let session = bluer::Session::new()
                    .await
                    .map_err(|_| BluetoothError::NoBluez)?;
                let adapter = session
                    .default_adapter()
                    .await
                    .map_err(|_| BluetoothError::NoAdapter)?;
                Ok::<Adapter, BluetoothError>(adapter)
            })
            .await
            .map_err(|_| BluetoothError::NoAdapter)??;
        Ok(Self {
            adapter: Arc::new(adapter),
            runtime,
        })
    }

    pub async fn get_status(&self) -> BluetoothStatus {
        let adapter = self.adapter.clone();
        self.runtime
            .spawn(async move {
                adapter.is_powered().await.map_or_else(
                    |_| BluetoothStatus::Unavailable,
                    |value| {
                        if value {
                            BluetoothStatus::Disconnected
                        } else {
                            BluetoothStatus::PoweredOff
                        }
                    },
                )
            })
            .await
            .unwrap_or(BluetoothStatus::Unavailable)
    }

    pub async fn scan(
        &self,
    ) -> Result<impl Stream<Item = DeviceDiscoveryEvent> + use<'_>, BluetoothError> {
        let adapter = self.adapter.clone();
        let stream = self
            .runtime
            .spawn(async move {
                let stream = adapter
                    .discover_devices_with_changes()
                    .await
                    .map_err(|_| BluetoothError::ScanFailed)?
                    .filter_map(move |item| {
                        let adapter = adapter.clone();
                        async move {
                            match item {
                                DeviceAdded(address) => {
                                    let device = BikeBt::get_device(adapter, address).await;
                                    match device {
                                        Ok(device) => {
                                            Some(DeviceDiscoveryEvent::DeviceAdded(device))
                                        }
                                        Err(_) => None,
                                    }
                                }
                                DeviceRemoved(address) => {
                                    let device = BikeBt::get_device(adapter, address).await;
                                    match device {
                                        Ok(device) => {
                                            Some(DeviceDiscoveryEvent::DeviceRemoved(device))
                                        }
                                        Err(_) => None,
                                    }
                                }
                                PropertyChanged(_) => None,
                            }
                        }
                    });
                Ok::<_, BluetoothError>(stream)
            })
            .await
            .map_err(|_| BluetoothError::ScanFailed)??;
        Ok(stream)

        //
        //Ok(adapter
        //    .discover_devices_with_changes()
        //    .await
        //    .map_err(|_| BluetoothError::ScanFailed)?
        //    .filter_map(move |item| async move {
        //        match item {
        //            DeviceAdded(address) => {
        //                let device = self.get_device(address).await;
        //                match device {
        //                    Ok(device) => Some(DeviceDiscoveryEvent::DeviceAdded(device)),
        //                    Err(_) => None,
        //                }
        //            }
        //            DeviceRemoved(address) => {
        //                let device = self.get_device(address).await;
        //                match device {
        //                    Ok(device) => Some(DeviceDiscoveryEvent::DeviceRemoved(device)),
        //                    Err(_) => None,
        //                }
        //            }
        //            PropertyChanged(_) => None,
        //        }
        //    }))
    }

    async fn get_device(adapter: Arc<Adapter>, address: Address) -> Result<Device, ()> {
        let device = adapter.device(address).map_err(|_| ())?;
        let name = device.name().await.map_err(|_| ())?.ok_or(())?;
        let rssi = device.rssi().await.ok().flatten().unwrap_or(-101_i16);
        Ok(Device::new(address.to_string(), name, rssi))
    }
}
