use std::sync::Arc;

use crate::components::active_device_details::{
    ActiveDeviceDetailsInput, ACTIVE_DEVICE_DETAILS_BROKER,
};
use crate::components::bluetooth_button::button::{AdapterStateInput, ADAPTER_STATE_BROKER};
use crate::components::connect_dialog::{DeviceDiscoveryEvent, DEVICE_DISCOVER_BROKER};
use bike_bt::{Address, BluetoothStatus, ConnectedDevice};
use futures::StreamExt;
use relm4::{
    gtk::glib::JoinHandle,
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    spawn_local,
};

pub struct StateManager {
    bike_bt: Arc<Option<bike_bt::BikeBt>>,
    connected_device: Option<ConnectedDevice>,
    scan_handler: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub enum StateManagerInput {
    RegisterAdapterListener,
    StartScanningForDevices,
    StopScanningForDevices,
    Connect(Address, String),
    Disconnect,
    DiscoverGattProfiles,
}

impl SimpleAsyncComponent for StateManager {
    type Input = StateManagerInput;
    type Output = ();
    type Init = ();
    type Root = ();
    type Widgets = ();

    fn init_root() -> Self::Root {}

    async fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: relm4::AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let bike_bt = bike_bt::BikeBt::new().await.ok();
        AsyncComponentParts {
            model: StateManager {
                bike_bt: Arc::new(bike_bt),
                connected_device: None,
                scan_handler: None,
            },
            widgets: (),
        }
    }

    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match message {
            StateManagerInput::RegisterAdapterListener => {
                if let Some(bike_bt) = self.bike_bt.as_ref() {
                    ADAPTER_STATE_BROKER
                        .send(AdapterStateInput::ChangeStatus(bike_bt.get_status().await));
                    match bike_bt.register_adapter_listener().await {
                        Ok(stream) => {
                            let stream = stream.for_each(|status| async {
                                ADAPTER_STATE_BROKER.send(AdapterStateInput::ChangeStatus(status));
                            });
                            spawn_local(async {
                                stream.await;
                            });
                        }
                        Err(error) => {
                            eprintln!("Could not regiester adapter listener. Error: {error}");
                        }
                    }
                }
            }
            StateManagerInput::StartScanningForDevices => {
                let bike_bt = Arc::clone(&self.bike_bt);
                self.scan_handler = Some(spawn_local(async move {
                    if let Some(bike_bt) = bike_bt.as_ref() {
                        match bike_bt.scan().await {
                            Ok(stream) => {
                                stream
                                    .for_each(|event| async {
                                        match event {
                                            bike_bt::DeviceDiscoveryEvent::DeviceAdded(device) => {
                                                DEVICE_DISCOVER_BROKER.send(
                                                    DeviceDiscoveryEvent::DeviceFound(device),
                                                );
                                            }
                                            bike_bt::DeviceDiscoveryEvent::DeviceRemoved(
                                                address,
                                            ) => {
                                                DEVICE_DISCOVER_BROKER.send(
                                                    DeviceDiscoveryEvent::DeviceLost(address),
                                                );
                                            }
                                        }
                                    })
                                    .await
                            }
                            Err(error) => {
                                eprintln!("Could not start scanning for devices. Error: {error}");
                            }
                        }
                    }
                }));
            }
            StateManagerInput::StopScanningForDevices => {
                self.scan_handler.take().inspect(|handler| handler.abort());
            }
            StateManagerInput::Connect(address, name) => {
                ACTIVE_DEVICE_DETAILS_BROKER.send(ActiveDeviceDetailsInput::SetName(name));
                if let Some(bike_bt) = self.bike_bt.as_ref() {
                    match bike_bt.connect(address).await {
                        Ok(connected_device) => {
                            self.connected_device = Some(connected_device);
                            ADAPTER_STATE_BROKER
                                .send(AdapterStateInput::ChangeStatus(BluetoothStatus::Connected));
                            ACTIVE_DEVICE_DETAILS_BROKER
                                .send(ActiveDeviceDetailsInput::SetConnected);
                        }
                        Err(error) => {
                            eprintln!("Could not connect to device. Error: {error}");
                            ACTIVE_DEVICE_DETAILS_BROKER
                                .send(ActiveDeviceDetailsInput::ConnectionFailed);
                        }
                    }
                }
            }
            StateManagerInput::Disconnect => {
                if let Some(connected_device) = &self.connected_device {
                    match connected_device.disconnect().await {
                        Ok(_) => {
                            ADAPTER_STATE_BROKER.send(AdapterStateInput::ChangeStatus(
                                                    BluetoothStatus::Disconnected,
                                                ));
                            self.connected_device = None;
                        },
                        Err(_) => {
                            eprintln!("Could not disconnect from device.")
                        }
                    }
                }
            }
            StateManagerInput::DiscoverGattProfiles => {
                if let Some(connected_device) = &self.connected_device {
                    match connected_device.get_gatt_services().await {
                        Ok(_) => {} //nothing to do
                        Err(_) => {
                            todo!("Implement fallback - disconnect device and navigate back")
                        }
                    }
                }
            }
        }
    }
}
