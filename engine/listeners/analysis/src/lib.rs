mod channel;
mod groups;
mod user;

use std::sync::Arc;
use telegram::client::report_error;
use telegram_types::{Client, Peer};

pub async fn run(
    client: Arc<Client>,
    bus: Arc<publisher::EventBus>,
    dialogs_data: Arc<dashmap::DashMap<i64, telegram::dialogs::DialogData>>,
    errors_peer: Peer,
) {
    let mut rx = bus.subscribe();

    println!("Telegram analyzer running...");

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let mut message_text = message.text().to_string();

        if message_text.is_empty() {
            message_text = "<non-text message (sticker or img)>\n Sent by: xxxxxxxxxx".to_string();
        }

        let msg_peer_id = message.peer_id().bare_id();

        let dialog_data = dialogs_data.get(&msg_peer_id);

        if dialog_data.is_none() {
            let client = client.clone();
            let errors_peer = errors_peer.clone();
            report_error(
                client,
                errors_peer,
                format!("Could not find dialog with ID: {}", msg_peer_id),
            );
            continue;
        }

        let dialog_data = dialog_data.unwrap();

        //TODO: handle form twitter bot

        let kind = &dialog_data.kind;
        let client = client.clone();
        let errors_peer = errors_peer.clone();

        match kind {
            telegram::dialogs::DialogType::User => {
                tokio::spawn(async move {
                    user::handle().await;
                });
            }
            telegram::dialogs::DialogType::Group => {
                tokio::spawn(async move {
                    groups::handle(&message).await;
                });
            }
            telegram::dialogs::DialogType::Channel => {
                tokio::spawn(async move {
                    channel::handle(client, &message, errors_peer).await;
                });
            }
        }
    }
}
