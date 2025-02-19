use relm4::MessageBroker;

use crate::state_manager::StateManagerInput;

//Incoming messages to the State Manager
pub static STATE_MANAGER: MessageBroker<StateManagerInput> = MessageBroker::new();
