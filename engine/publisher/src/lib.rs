use std::sync::Arc;

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum EventTag {
    Token,
    Other,
}

#[derive(Debug, Clone)]
pub struct TgEvent {
    pub chat: String,
    pub text: String,
    pub tag: EventTag,
}

pub async fn broadcast(
    bus: Arc<broadcast::Sender<TgEvent>>,
    chat: String,
    text: String,
    tag: EventTag,
) {
    let event = TgEvent { chat, text, tag };

    let _ = bus.send(event);
}

pub type EventBus = broadcast::Sender<TgEvent>;

pub fn new_event_bus() -> EventBus {
    let (tx, _) = broadcast::channel(1024);
    tx
}
