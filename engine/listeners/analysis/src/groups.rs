use std::sync::Arc;

use telegram_types::{Client, Message, Peer};

pub async fn handle(
    client: Arc<Client>,
    message: &Message,
    targeted_group_users_bare_ids: Vec<i64>,
    users_peer: Peer,
) {
    let message_text = message.text();
    let msg_sender = message.sender();

    let message_sender_bare_id = match msg_sender {
        Some(sender) => sender.id().bare_id(),
        None => {
            println!(
                "Something went wrong with the message sender. \nMessage text: {}\n PeerID: {} \n PeerBareID: {}",
                message_text,
                message.peer_id(),
                message.peer_id().bare_id()
            );
            return;
        }
    };

    if !targeted_group_users_bare_ids.contains(&message_sender_bare_id) {
        return;
    }

    let _ = client
        .send_message(
            users_peer,
            //TODO
            format!("{}\n---------\n{}", message_text, "SOMEONE IMPORTANT."),
        )
        .await;
}
