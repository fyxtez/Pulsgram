mod utils;
use publisher::EventBus;
use std::sync::Arc;
use telegram_types::{Client, Peer};
use twitter::regex::parse_message_type;

use crate::utils::handle_follow;

// TODO: Ignored senders implementation
pub async fn run(
    bus: Arc<EventBus>,
    client: Arc<Client>,
    target_dialog_id: i64,
    targeted_kols: Vec<String>,
    destination: Peer,
    source: Peer,
) {
    println!("KOL Follows running...");
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let peer_id = message.peer_id().bare_id();

        if !target_dialog_id.eq(&peer_id) {
            continue;
        }

        let message_text = message.text();

        let message_type = parse_message_type(message_text);

        dbg!(&message_text);
        dbg!(&message_type);

        handle_follow(&message_type, message, &client, &targeted_kols,&destination,&source).await;
    }
}
