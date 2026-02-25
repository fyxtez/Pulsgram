
use std::{
    io::{self},
    sync::Arc,
};

use crate::{config::load_tg_client_config, errors::TelegramError};
use grammers_client::{InvocationError, SignInError, client::UpdatesConfiguration};
use grammers_client::{peer::Peer, update::Update};
use grammers_mtsender::SenderPool;
use grammers_session::{storages::SqliteSession, updates::UpdatesLike};
use grammers_tl_types::enums::{InputNotifyPeer, InputPeer};
use tokio::sync::mpsc::UnboundedReceiver;

use grammers_client::Client;
pub struct ConnectClientReturnType {
    pub client: Client,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
}

async fn create_sender_pool(session_path: &str, api_id: i32) -> Result<SenderPool, TelegramError> {
    Ok(SenderPool::new(
        Arc::new(
            SqliteSession::open(session_path)
                .await
                .map_err(|e| TelegramError::Other(e.to_string()))?,
        ),
        api_id,
    ))
}

pub async fn connect_client(
    session_path: &str,
    api_id_var: &str,
    api_hash_var: &str,
    phone_number_var: &str,
    password_var: &str,
) -> Result<ConnectClientReturnType, TelegramError> {
    let config = load_tg_client_config(api_id_var, api_hash_var, phone_number_var, password_var)?;

    let sender_pool = create_sender_pool(session_path, config.api_id).await?;

    let client = Client::new(sender_pool.handle);

    tokio::spawn(sender_pool.runner.run());

    if !client.is_authorized().await? {
        let token = client
            .request_login_code(&config.phone_number, config.api_hash.as_str())
            .await?;

        println!("Enter the OTP code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, &config.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    println!(
        "Connected to Telegram via {}!",
        client.get_me().await?.full_name()
    );
    Ok(ConnectClientReturnType {
        client,
        updates_receiver: sender_pool.updates,
    })
}

pub async fn handle_updates(
    client: Arc<Client>,
    updates_receiver: UnboundedReceiver<UpdatesLike>,
    event_bus: Arc<publisher::EventBus>,
    dispatcher_user_id: i64,
) {
    let mut updates = client
        .stream_updates(
            updates_receiver,
            UpdatesConfiguration {
                catch_up: false,
                ..Default::default()
            },
        )
        .await;

    println!("Updates handler spawned.");

    loop {
        let update_result = updates.next().await;
        let update = match update_result {
            Ok(u) => u,
            Err(e) => {
                let msg = format!("Telegram update stream error: {}", e.to_string());

                let _ = event_bus.publish(publisher::types::PulsgramEvent::Error(
                    publisher::types::ErrorEvent {
                        message_text: msg,
                        source: "UpdateHandler::StreamError",
                    },
                ));
                continue;
            }
        };

        match update {
            Update::NewMessage(message) if !message.outgoing() => {
                let peer_id = message.peer_id().bare_id();

                if let Some(sender) = message.sender()
                    && sender.id().bare_id() == dispatcher_user_id
                {
                    continue;
                }

                // Ignore Pulsgram Errors channel
                if peer_id == 3228445189 {
                    //TODO
                    continue;
                }

                event_bus.publish(publisher::types::PulsgramEvent::Telegram(
                    publisher::types::TgEvent {
                        message: Box::new(message),
                    },
                ));
            }
            _ => {}
        }
    }
}

//TODO: Test this v0.9.0 update.
pub async fn toggle_mute_peer(
    client: Arc<Client>,
    peer: &Peer,
    mute: bool,
) -> Result<(), InvocationError> {
    let peer_ref = peer
        .to_ref()
        .await
        .ok_or_else(|| InvocationError::Dropped)?;
    let input_peer: InputPeer = peer_ref.into();

    let notify_peer =
        InputNotifyPeer::Peer(grammers_tl_types::types::InputNotifyPeer { peer: input_peer });

    let ipns = grammers_tl_types::types::InputPeerNotifySettings {
        show_previews: Some(false),
        silent: Some(false),
        mute_until: Some(if mute { i32::MAX } else { i32::MIN }),
        sound: None,
        stories_muted: Some(false),
        stories_hide_sender: Some(false),
        stories_sound: Default::default(),
    };

    let settings = grammers_tl_types::enums::InputPeerNotifySettings::Settings(ipns);

    let _x = client
        .invoke(
            &grammers_tl_types::functions::account::UpdateNotifySettings {
                peer: notify_peer,
                settings,
            },
        )
        .await?;
    Ok(())
}
