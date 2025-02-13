#[derive(Default, Debug, PartialEq)]
pub enum BluetoothStatus {
    #[default]
    Unavailable,
    PoweredOff,
    Disconnected,
    Connected,
}


