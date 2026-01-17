use std::sync::Arc;

use publisher::EventBus; //TODO: This shouldnt be directly from publisher rather from a shared crate

pub async fn run(bus: Arc<EventBus>) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        println!("[TEST_LISTENER] {}", event.text);
    }
}
