use grammers_client::Client;
use grammers_client::peer::Dialog;
use grammers_client::peer::Peer;
use grammers_session::types::PeerRef;
use serde::Serialize;
use std::collections::HashMap;

use crate::errors::TelegramError;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DialogType {
    User = 0,
    Group = 1,
    Channel = 2,
}

#[derive(Debug, Serialize, Clone)]
pub struct DialogData {
    pub name: String,
    pub username: Option<String>,
    pub kind: DialogType,
    pub bare_id: i64,
}

pub fn get_dialog_type(dialog: &Dialog) -> DialogType {
    match dialog.peer {
        Peer::User(_) => DialogType::User,
        Peer::Group(_) => DialogType::Group,
        Peer::Channel(_) => DialogType::Channel,
    }
}

pub async fn load_dialogs(client: &Client) -> Result<Vec<Dialog>, TelegramError> {
    let mut iter_dialogs = client.iter_dialogs();

    let mut dialogs: Vec<Dialog> = Vec::new();

    while let Some(dialog) = iter_dialogs.next().await? {
        dialogs.push(dialog);
    }
    println!("Telegram dialogs loaded.");

    Ok(dialogs)
}

pub async fn build_peers_map_from_dialogs(dialogs: &[Dialog]) -> HashMap<i64, PeerRef> {
    let mut map = HashMap::new();

    for dialog in dialogs {
        let peer = dialog.peer();
        let bare_id = peer.id().bare_id();

        if let Some(peer_ref) = peer.to_ref().await {
            map.insert(bare_id, peer_ref);
        } else {
            eprintln!("Failed to resolve peer {}", bare_id);
        }
    }

    println!("Peers map loaded (PeerRef).");

    map
}

pub fn print_dialogs(dialogs: &[Dialog]) {
    for dialog in dialogs {
        let dialog_type = get_dialog_type(dialog);
        let peer = dialog.peer();
        let id = peer.id().bare_id();

        let name = match peer {
            Peer::User(user) => user.username().unwrap_or("No username"),
            Peer::Group(group) => group.title().unwrap_or("No title"),
            Peer::Channel(channel) => channel.title(),
        };

        println!("Dialog ID: {}, Name: {}, Type: {:?}", id, name, dialog_type);
    }
}

pub fn print_peer_data(peer: &Peer) {
    let id = peer.id().bare_id();

    match peer {
        Peer::User(user) => {
            println!("User Peer - ID: {}, Username: {:?}", id, user.username());
        }
        Peer::Group(group) => {
            println!("Group Peer - ID: {}, Title: {:?}", id, group.title());
        }
        Peer::Channel(channel) => {
            println!("Channel Peer - ID: {}, Title: {:?}", id, channel.title());
        }
    }
}

pub fn peer_to_dialog_data(peer: &Peer) -> (i64, DialogData) {
    let id = peer.id().bare_id();
    match peer {
        Peer::User(user) => {
            let data = DialogData {
                name: user.first_name().unwrap_or("Unnamed user").to_string(),
                username: user.username().map(|u| u.to_string()),
                kind: DialogType::User,
                bare_id: id,
            };

            (id, data)
        }

        Peer::Group(group) => {
            let data = DialogData {
                name: group.title().unwrap_or("Unnamed group").to_string(),
                username: None,
                kind: DialogType::Group,
                bare_id: id,
            };

            (id, data)
        }

        Peer::Channel(channel) => {
            let data = DialogData {
                name: channel.title().to_string(),
                username: channel.username().map(|u| u.to_string()),
                kind: DialogType::Channel,
                bare_id: id,
            };

            (id, data)
        }
    }
}

pub fn normalize_dialogs_into_data(dialogs: &[Dialog]) -> dashmap::DashMap<i64, DialogData> {
    let dialogs_data = dashmap::DashMap::new();

    for dialog in dialogs {
        let peer = dialog.peer();
        let (id, dialog_data) = peer_to_dialog_data(peer);

        dialogs_data.insert(id, dialog_data);
    }

    println!("Dialogs normalized");

    dialogs_data
}

//. TODO: Endpoint
pub async fn clear_dialogs(
    client: &Client,
    dialogs: &[Dialog],
    ignore_users: bool,
) -> Result<(), TelegramError> {
    for dialog in dialogs {
        let peer = dialog.peer();

        let peer_ref = peer.to_ref().await.ok_or_else(|| {
            TelegramError::Other(format!(
                "Could not convert to peer ref | id={} name='{}' username={:?}",
                peer.id(),
                peer.name().unwrap_or("Peer has no name."),
                peer.username(),
            ))
        })?;
        if !ignore_users && let Peer::User(_) = peer {
            continue;
        }

        client.mark_as_read(peer_ref).await?;
    }

    Ok(())
}
