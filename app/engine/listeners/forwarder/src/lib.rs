use std::sync::Arc;

use publisher::handle_recv_error;
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
                        Ok(_) => {}
                        Err(error) => {
                            let msg = format!(
                                "Failed to forward message.\nFrom: {}\nTo: {}\nError: {}",
                                from_peer.id, to_peer.id, error
                            );

                            // We intentionally ignore the result of publish() here.
                            // If the error bus is closed or unavailable, there is nothing this worker
                            // can safely do about it. Forwarder must not panic or block on error reporting.
                            // The dedicated error listener is responsible for handling reporting failures.

                            bus.publish(publisher::types::PulsgramEvent::Error(
                                publisher::types::ErrorEvent {
                                    message_text: msg,
                                    source: "Forwarder::Err",
                                },
                            ));
                        }
                    }
                }
                _ => continue,
            },
            Err(error) => {
                if handle_recv_error("Forwarder RecvError", error, &bus) {
                    break;
                }
            }
        }
    }
}
