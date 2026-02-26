pub mod types;

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

use crate::types::PulsgramEvent;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<PulsgramEvent>,
}
impl EventBus {
    pub fn publish(&self, event: PulsgramEvent) {
        if let Err(_) = self.sender.send(event) {
            eprintln!("EventBus: no active receivers");
            println!("EventBus: no active receivers");
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PulsgramEvent> {
        self.sender.subscribe()
    }
}

pub fn new_event_bus() -> EventBus {
    let (sender, _) = broadcast::channel(1024);
    EventBus { sender }
}

pub fn handle_recv_error(source: &'static str, error: RecvError, bus: &EventBus) -> bool {
    let msg = match error {
        RecvError::Closed => "Broadcast channel closed".to_string(),
        RecvError::Lagged(n) => {
            format!("Broadcast lagged by {} messages", n)
        }
    };

    bus.publish(types::PulsgramEvent::Error(types::ErrorEvent {
        message_text: msg,
        source,
    }));

    matches!(error, RecvError::Closed)
}
