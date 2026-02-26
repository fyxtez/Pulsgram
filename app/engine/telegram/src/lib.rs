use grammers_client::{
    Client,
    peer::{Peer, User},
};

use crate::errors::TelegramError;

pub mod client;
pub mod config;
pub mod dialogs;
pub mod errors;
pub mod media;
pub mod msg_reply;

pub async fn get_me(client: &Client) -> Result<User, TelegramError> {
    Ok(client.get_me().await?)
}
pub async fn resolve_username(client: &Client, username: &str) -> Result<Peer, TelegramError> {
    let peer = client.resolve_username(username).await?;
    match peer {
        Some(peer) => Ok(peer),
        None => {
            return Err(TelegramError::Other(format!(
                "Could not resolve username: {}",
                username
            )));
        }
    }
}
