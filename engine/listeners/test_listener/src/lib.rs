use std::sync::Arc;

use publisher::EventBus;

pub async fn run(bus: Arc<EventBus>) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        println!("[TEST_LISTENER] {}", event.message.text());
    }
}
