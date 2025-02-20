use crate::{
    bluetooth_error::DeviceConnectionError, BluetoothError, BluetoothStatus, ConnectedDevice,
    Device, DeviceDiscoveryEvent,
};
use bluer::{
    AdapterEvent::{DeviceAdded, DeviceRemoved, PropertyChanged},
    AdapterProperty, Address,
};
use futures::{stream::StreamExt, Stream};

pub struct BikeBt {
    adapter: bluer::Adapter,
    pub device: Option<Device>,
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
        Ok(Self {
            adapter,
            device: None,
        })
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

    pub async fn scan(
        &self,
    ) -> Result<impl Stream<Item = DeviceDiscoveryEvent> + use<'_>, BluetoothError> {
        let stream = self
            .adapter
            .discover_devices()
            .await
            .map_err(|_| BluetoothError::ScanFailed)?
            .filter_map(|itm| async {
                let item = itm;
                match item {
                    DeviceAdded(address) => self
                        .get_device(address)
                        .await
                        .map(|device| DeviceDiscoveryEvent::DeviceAdded(device)),
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

    pub async fn get_device(&self, address: Address) -> Option<Device> {
        match self.adapter.device(address).map_err(|_| ()) {
            Ok(device) => Device::new(device).await,
            Err(_) => None,
        }
    }

    pub async fn connect(
        &self,
        address: Address,
    ) -> Result<ConnectedDevice, DeviceConnectionError> {
        let device = self
            .adapter
            .device(address)
            .map_err(|error| DeviceConnectionError::new(error.message))?;

        if !device
            .is_trusted()
            .await
            .map_err(|e| DeviceConnectionError::new(e.message))?
        {
            device
                .set_trusted(true)
                .await
                .map_err(|e| DeviceConnectionError::new(e.message))?;
        }

        if !device
            .is_paired()
            .await
            .map_err(|error| DeviceConnectionError::new(error.message))?
        {
            device
                .pair()
                .await
                .map_err(|error| DeviceConnectionError::new(error.message))?;
        }

        if !device
            .is_connected()
            .await
            .map_err(|e| DeviceConnectionError::new(e.message))?
        {
            device
                .connect()
                .await
                .map_err(|error| DeviceConnectionError::new(error.message))?;
        }

        let name = device
            .name()
            .await
            .map_err(|error| DeviceConnectionError::new(error.message))?
            .ok_or(DeviceConnectionError::new("Device has no name".to_string()))?;
        Ok(ConnectedDevice::new(device, name))
    }
}
