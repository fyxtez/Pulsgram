pub mod types;

use tokio::sync::broadcast;

use crate::types::PulsgramEvent;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<PulsgramEvent>,
}
impl EventBus {
    pub fn publish(&self, event: PulsgramEvent) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PulsgramEvent> {
        self.sender.subscribe()
    }
}

pub fn new_event_bus() -> EventBus {
    let (sender, _) = broadcast::channel(1024);
    EventBus { sender }
}
