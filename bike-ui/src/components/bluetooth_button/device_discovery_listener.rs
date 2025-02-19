use bike_bt::{Address, Device};
use relm4::{ComponentParts, MessageBroker, SimpleComponent};

pub static DEVICE_DISCOVER_BROKER: MessageBroker<DeviceDiscoveryEvent> = MessageBroker::new();

pub struct DeviceDiscoveryListener;

#[derive(Debug)]
pub enum DeviceDiscoveryEvent {
    DeviceFound(Device),
    DeviceLost(Address),
}

impl SimpleComponent for DeviceDiscoveryListener {
    type Input = DeviceDiscoveryEvent;

    type Output = DeviceDiscoveryEvent;

    type Init = ();

    type Root = ();

    type Widgets = ();

    fn init_root() -> Self::Root {}

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        ComponentParts {
            model: Self {},
            widgets: (),
        }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        let _ = match message {
            DeviceDiscoveryEvent::DeviceFound(device) => {
                sender.output(DeviceDiscoveryEvent::DeviceFound(device))
            }
            DeviceDiscoveryEvent::DeviceLost(address) => {
                sender.output(DeviceDiscoveryEvent::DeviceLost(address))
            }
        };
    }
}
