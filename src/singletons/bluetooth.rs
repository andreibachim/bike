use once_cell::sync::Lazy;

pub struct Bluetooth {
    state: ConnectionState,
}

#[derive(Debug)]
pub enum ConnectionState {
    NotAvailable,
}

impl Bluetooth {
    fn new() -> Self {
        Bluetooth {
            state: ConnectionState::NotAvailable,
        }
    }

    pub fn get_state(&self) -> &ConnectionState {
        &self.state
    }
}

pub static BLUETOOTH: Lazy<Bluetooth> = Lazy::new(|| Bluetooth::new());
