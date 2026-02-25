mod error;
use std::sync::Arc;
use publisher::handle_recv_error;
use telegram_types::Client;
use telegram_types::PeerRef;

pub async fn run(client: Arc<Client>, to_peer: PeerRef, bus: Arc<publisher::EventBus>) {
    println!("Errors Reporter running...");
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Error(error_event) => {
                    let error_message = error_event.message_text;

                    error::report_error(
                        &client,
                        to_peer,
                        "Errors Reporter Listener::Ok",
                        &error_message,
                    )
                    .await;
                }
                _ => continue,
            },
            Err(error) => {
                if handle_recv_error("Errors Reporter::Err", error, &bus) {
                    break;
                }
            }
        }
    }
}
