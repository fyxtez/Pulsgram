use std::{
    error::Error,
    io::{self},
    sync::Arc,
};

use crate::config::load_tg_client_config;
use grammers_client::{
    InvocationError, SignInError, UpdatesConfiguration,
    grammers_tl_types::{
        self,
        enums::{InputNotifyPeer, InputPeer},
    },
};
use grammers_client::{Update, types::Peer};
use grammers_mtsender::SenderPool;
use grammers_session::{defs::PeerRef, storages::SqliteSession, updates::UpdatesLike};
use tokio::sync::mpsc::UnboundedReceiver;

use grammers_client::Client;
pub struct ConnectClientReturnType {
    pub client: Client,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
}

fn create_sender_pool(session_path: &str, api_id: i32) -> Result<SenderPool, Box<dyn Error>> {
    let session = SqliteSession::open(session_path)?;

    let sender_pool = SenderPool::new(std::sync::Arc::new(session), api_id);

    Ok(sender_pool)
}

pub async fn connect_client(session_path: &str) -> Result<ConnectClientReturnType, Box<dyn Error>> {
    let config = load_tg_client_config()?;

    let sender_pool = create_sender_pool(session_path, config.api_id)?;

    let client = Client::new(&sender_pool);

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
        client.get_me().await.unwrap().full_name()
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
) {
    let mut updates = client.stream_updates(
        updates_receiver,
        UpdatesConfiguration {
            catch_up: false,
            ..Default::default()
        },
    );

    loop {
        let update = updates.next().await;
        let update = update.unwrap(); // TODO: handle error properly
        match update {
            Update::NewMessage(message) if !message.outgoing() => {
                // TODO: Probably handle sniper here too.

                tokio::spawn(publisher::broadcast(
                    event_bus.clone(),
                    message,
                    publisher::types::EventTag::Other,
                ));
            }
            _ => {
                // println!("Other update: {:?}", update);
            }
        }
    }
}

pub fn report_error(client: Arc<Client>, errors_peer: Peer, message: String) {
    tokio::spawn(async move {
        if let Err(e) = client.send_message(errors_peer, message).await {
            eprintln!("Failed to send error message: {e}");
        }
    });
}

pub async fn toggle_mute_peer(
    client: Arc<Client>,
    peer: &Peer,
    mute: bool,
) -> Result<(), InvocationError> {
    let peer_ref = PeerRef::from(peer);
    let input_peer: InputPeer = peer_ref.into();

    let notify_peer =
        InputNotifyPeer::Peer(grammers_tl_types::types::InputNotifyPeer { peer: input_peer });

    let mute_until = match mute {
        true => Some(i32::MAX),
        false => Some(i32::MIN),
    };
    let ipns = grammers_tl_types::types::InputPeerNotifySettings {
        show_previews: Some(false),
        silent: Some(false),
        mute_until,
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
