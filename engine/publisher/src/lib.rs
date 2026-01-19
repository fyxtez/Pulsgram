pub mod types;

use std::sync::Arc;

use grammers_client::types::update::Message;
use tokio::sync::broadcast;

use crate::types::{EventTag, TgEvent};

pub async fn broadcast(bus: Arc<broadcast::Sender<TgEvent>>, message: Message, tag: EventTag) {
    let event = TgEvent { message, tag };

    let _ = bus.send(event);
}

pub type EventBus = broadcast::Sender<TgEvent>;

pub fn new_event_bus() -> EventBus {
    let (tx, _) = broadcast::channel(1024);
    tx
}
