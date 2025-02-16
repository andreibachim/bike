use bluer::Address;

#[derive(Debug)]
pub struct Device {
    pub address: Address,
    pub name: String,
    pub paired: bool,
    pub signal: DeviceSignalStrength,
}

impl Device {
    pub fn new(address: Address, name: String, paired: bool, signal: i16) -> Self {
        Self {
            address,
            name,
            paired,
            signal: DeviceSignalStrength::from(signal),
        }
    }
}

#[derive(Debug)]
pub enum DeviceDiscoveryEvent {
    DeviceAdded(Device),
    DeviceRemoved(Address),
}

#[derive(Debug)]
pub enum DeviceSignalStrength {
    NoSignal,
    Weak,
    Ok,
    Good,
    Full,
}

impl DeviceSignalStrength {
    fn from(value: i16) -> Self {
        match value {
            i16::MIN..=-120 => Self::NoSignal,
            -119..=-90 => Self::Weak,
            -89..=-60 => Self::Ok,
            -59..=-30 => Self::Good,
            -29..=0 => Self::Full,
            _ => Self::NoSignal,
        }
    }
}
