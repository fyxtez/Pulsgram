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

    while let Ok(event) = rx.recv().await {
        let message = event.message;
        let message_peer_id = message.peer_id();

        if message_peer_id != from_peer.id {
            continue;
        }

        let mut message_text = message.text().to_string();

        if message_text.is_empty() {
            message_text = "<non-text message (sticker or img)>\n".to_string();
        }

        //TODO: Add message sender/peer who sent the message.
        let _ = client.send_message(to_peer, message_text).await; //TODO: Handle error
    }
}
