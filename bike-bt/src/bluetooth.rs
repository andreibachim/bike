use crate::{BluetoothError, BluetoothStatus, Device, DeviceDiscoveryEvent};
use bluer::{
    AdapterEvent::{DeviceAdded, DeviceRemoved, PropertyChanged},
    AdapterProperty, Address,
};
use futures::{stream::StreamExt, Stream};

pub struct BikeBt {
    pub session: bluer::Session,
    pub adapter: bluer::Adapter,
}

impl BikeBt {
    pub async fn new() -> Result<Self, BluetoothError> {
        let session = bluer::Session::new()
            .await
            .map_err(|_| BluetoothError::NoBluez)?;

        let adapter = session
            .default_adapter()
            .await
            .map_err(|_| BluetoothError::NoAdapter)?;
        Ok(Self { session, adapter })
    }

    pub async fn get_status(&self) -> BluetoothStatus {
        self.adapter.is_powered().await.map_or_else(
            |_| BluetoothStatus::Unavailable,
            |powered| {
                if powered {
                    BluetoothStatus::Disconnected
                } else {
                    BluetoothStatus::PoweredOff
                }
            },
        )
    }

    pub async fn scan(&self) -> Result<impl Stream<Item = DeviceDiscoveryEvent>, BluetoothError> {
        let stream = self
            .adapter
            .discover_devices()
            .await
            .map_err(|_| BluetoothError::ScanFailed)?
            .filter_map(move |item| async move {
                match item {
                    DeviceAdded(address) => Some(DeviceDiscoveryEvent::DeviceAdded(address)),
                    DeviceRemoved(address) => Some(DeviceDiscoveryEvent::DeviceRemoved(address)),
                    PropertyChanged(_) => None,
                }
            });
        Ok::<_, BluetoothError>(stream)
    }

    pub async fn register_adapter_listener(
        &self,
    ) -> Result<impl Stream<Item = BluetoothStatus>, BluetoothError> {
        let stream = self
            .adapter
            .events()
            .await
            .map_err(|_| BluetoothError::ScanFailed)?
            .filter_map(move |event| async {
                match event {
                    DeviceAdded(_) => None,
                    DeviceRemoved(_) => None,
                    PropertyChanged(adapter_property) => match adapter_property {
                        AdapterProperty::Powered(value) => Some(match value {
                            true => BluetoothStatus::Disconnected,
                            false => BluetoothStatus::PoweredOff,
                        }),
                        _ => None,
                    },
                }
            });
        Ok(stream)
    }

    pub async fn get_device(&self, address: Address) -> Result<Device, ()> {
        let device = self.adapter.device(address).map_err(|_| ())?;
        let name = device.name().await.map_err(|_| ())?.ok_or(())?;
        let rssi = device.rssi().await.ok().flatten().unwrap_or(-101_i16);
        Ok(Device::new(address.to_string(), name, rssi))
    }
}
