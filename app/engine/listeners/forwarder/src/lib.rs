use std::sync::Arc;

use telegram_types::Client;
use telegram_types::PeerRef;

pub async fn run(
    client: Arc<Client>,
    from_peer: PeerRef,
    to_peer: PeerRef,
    bus: Arc<publisher::EventBus>,
) {
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Telegram(tg_event) => {
                    let message = tg_event.message;
                    let message_peer_id = message.peer_id();

                    if message_peer_id != from_peer.id {
                        continue;
                    }

                    let mut message_text = message.text().to_owned();

                    if message_text.is_empty() {
                        message_text = "<non-text message (sticker or img)>\n".to_string();
                    }

                    match client.send_message(to_peer, message_text).await {
                        Ok(_) => continue,
                        Err(_error) => {
                            //TODO: Report error wif out publishing agian.
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
