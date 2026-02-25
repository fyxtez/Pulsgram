use std::sync::Arc;

use telegram_types::Client;
use telegram_types::PeerRef;

pub async fn run(client: Arc<Client>, to_peer: PeerRef, bus: Arc<publisher::EventBus>) {
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Error(error_event) => {
                    let error_message = error_event.message_text;

                    match client.send_message(to_peer, error_message).await {
                        Ok(_) => continue,
                        Err(_error) => {
                            // TODO: Report error wif out publishing agian.
                        }
                    }
                }
                _ => continue,
            },
            Err(error) => {
                println!("Error receiving event: {:?}", error);
                continue;
            }
        }
    }
}
