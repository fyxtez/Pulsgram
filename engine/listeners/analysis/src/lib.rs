mod channel;
mod groups;
mod keywords;
mod user;

use std::sync::Arc;
use telegram::{client::report_error, dialogs::DialogData};
use telegram_types::{Client, Peer};


//TODO: Too many arguments... donst look good.
pub async fn run(
    client: Arc<Client>,
    bus: Arc<publisher::EventBus>,
    dialogs_data: Arc<dashmap::DashMap<i64, telegram::dialogs::DialogData>>,
    errors_peer: Peer,
    users_group_peer: Peer,
    targeted_users: Vec<DialogData>,
    targeted_group_users: Vec<i64>,
    targeted_channels: Vec<i64>,
) {
    let mut rx = bus.subscribe();

    println!("Telegram analyzer running...");

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let mut message_text = message.text().to_string();

        if message_text.is_empty() {
            message_text = "<non-text message (sticker or img)>\n Sent by: xxxxxxxxxx".to_string();

            continue;
        }

        let msg_peer_id = message.peer_id().bare_id();

        let dialog_data = dialogs_data.get(&msg_peer_id);

        // Uknown dialog sent a message.
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

        //TODO: Pretty ugly in a while loop.
        //These should be mostly Arc which come from state, so theres no deep copying just ref count increase.
        let kind = &dialog_data.kind;
        let client = client.clone();
        let errors_peer = errors_peer.clone();
        let targeted_users = targeted_users.clone();
        let targeted_group_users = targeted_group_users.clone();
        let users_group_peer = users_group_peer.clone();
        let targeted_channels = targeted_channels.clone();

        match kind {
            telegram::dialogs::DialogType::User => {
                tokio::spawn(async move {
                    user::handle(client, &message, targeted_users, users_group_peer).await;
                });
            }
            telegram::dialogs::DialogType::Group => {
                tokio::spawn(async move {
                    groups::handle(client, &message, targeted_group_users, users_group_peer).await;
                });
            }
            telegram::dialogs::DialogType::Channel => {
                tokio::spawn(async move {
                    channel::handle(
                        client,
                        &message,
                        errors_peer,
                        targeted_channels,
                        users_group_peer,
                    )
                    .await;
                });
            }
        }
    }
}
