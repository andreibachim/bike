use std::fmt;
use BluetoothError::*;

#[derive(Debug)]
pub enum BluetoothError {
    NoBluez,
    NoAdapter,
    ScanFailed,
}

impl fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoBluez => write!(f, "Could not init bluez session"), 
            NoAdapter => write!(f, "Could not find any bluetooth adapters"),
            ScanFailed => write!(f, "Could not scan for bluetooth devices"),
        } 
    }
}

impl std::error::Error for BluetoothError {}


