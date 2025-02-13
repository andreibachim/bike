#[derive(Debug)]
pub struct Device {
    pub address: String,
    pub name: String,
    pub signal: DeviceSignalStrength,
}

impl Device {
    pub fn new(address: String, name: String, signal: i16) -> Self {
        Self {
            address,
            name,
            signal: DeviceSignalStrength::from(signal),
        }
    }
}

pub enum DeviceDiscoveryEvent {
    DeviceAdded(Device),
    DeviceRemoved(Device),
}

#[derive(Debug)]
pub enum DeviceSignalStrength {
    NoSignal,
    Weak,
    Medium,
    Full,
}

impl DeviceSignalStrength {
    fn from(value: i16) -> Self {
        match value {
            i16::MIN..=-100 => Self::NoSignal,
            -99..=-70 => Self::Weak,
            -69..=-40 => Self::Medium,
            -39..=0 => Self::Full,
            _ => Self::NoSignal,
        }
    }
}
