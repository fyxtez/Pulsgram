use std::{
    error::Error,
    io::{self},
    sync::Arc,
};

use grammers_client::Update;
use grammers_client::{Client, SignInError, UpdatesConfiguration};
use grammers_mtsender::SenderPool;
use grammers_session::{storages::SqliteSession, updates::UpdatesLike};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::config::load_tg_client_config;

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
                println!("Message is {}", message.text());

                // TODO
                // if let Ok(peer) = message.peer() {
                //     let msg_peer_name = peer.name();
                //     let msg_peer_id = peer.id();

                //     println!("Peer: name={:?}, id={:?}", msg_peer_name, msg_peer_id);
                // } else {
                //     println!("Could not get peer info");
                // }

                let peer_id = message.peer_id();

                // let message_id = message.id();
                // println!("Message ID: {}", message_id);

                // if let Some(sender) = message.sender() {
                //     let msg_sender_id = sender.id();
                //     let msg_sender_name = sender.name();
                //     let msg_sender_username = sender.username();

                //     println!(
                //         "Sender: name={:?}, id={}, username={:?}",
                //         msg_sender_name, msg_sender_id, msg_sender_username
                //     );

                //     println!("Sender (via message.sender()): id={}", msg_sender_id);
                // } else {
                //     println!("Message has NO sender");
                // }

                println!("Peer ID: {:?}", peer_id);

                tokio::spawn(publisher::broadcast(
                    event_bus.clone(),
                    message,
                    publisher::EventTag::Other,
                ));
            }
            _ => {
                // println!("Other update: {:?}", update);
            }
        }
    }
}
