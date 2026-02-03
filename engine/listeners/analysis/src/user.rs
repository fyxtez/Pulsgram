use std::sync::Arc;

use telegram::dialogs::{DialogData, find_dialog_data_by_bare_id};
use telegram_types::{Client, Message, Peer};

pub async fn handle(
    client: Arc<Client>,
    message: &Message,
    targeted_users_dialog_data: Vec<DialogData>,
    users_peer: Peer,
) {
    let message_text = message.text();
    let message_peer_bare_id = message.peer_id().bare_id();
    let dialog_data =
        find_dialog_data_by_bare_id(&targeted_users_dialog_data, message_peer_bare_id);
    if dialog_data.is_none() {
        return;
    }

    //TODO: Handle error.
    let _ = client
        .send_message(
            users_peer,
            format!("{}\n---------\n{}", message_text, dialog_data.unwrap().name),
        )
        .await;
}
