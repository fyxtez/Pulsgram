use std::sync::Arc;

use telegram::client::report_error;
use telegram_types::{Client, Message, Peer};

pub async fn handle(
    client: Arc<Client>,
    message: &Message,
    errors_peer: Peer,
    targeted_channels: Vec<i64>,
    users_peer: Peer,
) {
    let peer = message.peer();
    let message_text = message.text();
    let message_peer_bare_id = message.peer_id().bare_id();

    if !targeted_channels.contains(&message_peer_bare_id) {
        dbg!("nonono");
        return;
    }

    if peer.is_err() {
        report_error(
            client.clone(),
            errors_peer,
            format!(
                "Something went wrong fetching peer. Message: {}",
                message.text()
            ),
        );
    }

    let send_message_result = client
        .send_message(
            users_peer,
            format!(
                "{}\n---------\n{}",
                message_text,
                message.peer().unwrap().name().unwrap_or("Unnamed Channel")
            ),
        )
        .await;

    //TODO: Report error
    if send_message_result.is_err() {
        println!("-------");
        println!("{:?}", send_message_result.err());
    }
}
