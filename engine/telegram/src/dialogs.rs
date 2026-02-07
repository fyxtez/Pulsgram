use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use grammers_client::Client;
use grammers_client::InvocationError;
use grammers_client::types::Dialog;
use grammers_client::types::Peer;
use serde::Serialize;

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

//TODO: Use built in find using iter.
pub fn find_dialog_data_by_bare_id(
    dialogs: &[DialogData],
    target_bare_id: i64,
) -> Option<&DialogData> {
    for dialog_data in dialogs {
        if dialog_data.bare_id.eq(&target_bare_id) {
            return Some(dialog_data);
        }
    }
    None
}

pub async fn load_dialogs(client: &Client) -> Result<Vec<Dialog>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut dialogs: Vec<Dialog> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            dialogs.push(next_dialog);
        }
    }

    Ok(dialogs)
}

pub async fn get_peer_by_bare_id(
    client: &Client,
    bare_id_to_find: i64,
) -> Result<Option<Peer>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    while let Some(dialog) = iter_dialogs.next().await? {
        let dialog_bare_id = match &dialog.peer {
            Peer::User(user) => user.bare_id(),
            Peer::Group(group) => group.id().bare_id(),
            Peer::Channel(channel) => channel.bare_id(),
        };

        if dialog_bare_id == bare_id_to_find {
            return Ok(Some(dialog.peer));
        }
    }

    Ok(None)
}

pub fn build_peers_map_from_dialogs(dialogs: &[Dialog]) -> HashMap<i64, Peer> {
    let mut map = HashMap::new();

    for dialog in dialogs {
        let bare_id = match &dialog.peer {
            Peer::User(user) => user.bare_id(),
            Peer::Group(group) => group.id().bare_id(),
            Peer::Channel(channel) => channel.bare_id(),
        };
        map.insert(bare_id, dialog.peer.clone());
    }

    map
}

pub async fn get_peers_by_bare_ids(
    client: &Client,
    bare_ids: Vec<i64>,
) -> Result<HashMap<i64, Peer>, InvocationError> {
    let wanted: HashSet<i64> = bare_ids.into_iter().collect();
    let mut found: HashMap<i64, Peer> = HashMap::new();

    let mut iter_dialogs = client.iter_dialogs();

    while let Some(dialog) = iter_dialogs.next().await? {
        let peer = dialog.peer;

        let bare_id = match &peer {
            Peer::User(user) => user.bare_id(),
            Peer::Group(group) => group.id().bare_id(),
            Peer::Channel(channel) => channel.bare_id(),
        };

        if wanted.contains(&bare_id) {
            found.insert(bare_id, peer);

            // early exit if we already found everything
            if found.len() == wanted.len() {
                break;
            }
        }
    }

    Ok(found)
}

pub async fn get_peer(
    client: &Arc<Client>,
    bare_id: i64,
) -> Result<Peer, Box<dyn std::error::Error>> {
    get_peer_by_bare_id(client, bare_id)
        .await?
        .ok_or_else(|| format!("Could not find peer with ID: {}", bare_id).into())
}

pub fn print_dialogs(dialogs: &Vec<Dialog>) -> Result<(), InvocationError> {
    for dialog in dialogs {
        let dialog_type = get_dialog_type(dialog);
        let peer = dialog.peer();

        let (id, name) = match peer {
            Peer::User(user) => (user.bare_id(), user.username().unwrap_or("No username")),
            Peer::Group(group) => (group.id().bare_id(), group.title().unwrap_or("No title")),
            Peer::Channel(channel) => (channel.bare_id(), channel.title()),
        };

        println!("Dialog ID: {}, Name: {}, Type: {:?}", id, name, dialog_type);
    }

    Ok(())
}

pub fn print_peer_data(peer: &Peer) {
    match peer {
        Peer::User(user) => {
            println!(
                "User Peer - ID: {}, Username: {:?}",
                user.bare_id(),
                user.username()
            );
        }
        Peer::Group(group) => {
            println!(
                "Group Peer - ID: {}, Title: {:?}",
                group.id().bare_id(),
                group.title()
            );
        }
        Peer::Channel(channel) => {
            println!(
                "Channel Peer - ID: {}, Title: {:?}",
                channel.bare_id(),
                channel.title()
            );
        }
    }
}


pub fn peer_to_dialog_data(peer: &Peer) -> (i64, DialogData) {
    let peer_name = peer.name().unwrap_or("Unnamed");
    match peer {
        Peer::User(user) => {
            let id = user.bare_id();

            let data = DialogData {
                name: user.first_name().unwrap_or("Unnamed user").to_string(),
                username: user.username().map(|u| u.to_string()),
                kind: DialogType::User,
                bare_id: id,
            };

            (id, data)
        }

        Peer::Group(group) => {
            let id = group.id().bare_id();
             
            if peer_name.eq("Trenches Bunker Messages") {
                dbg!("1111");
            }

            let data = DialogData {
                name: group.title().unwrap_or("Unnamed group").to_string(),
                username: None,
                kind: DialogType::Group,
                bare_id: id,
            };

            (id, data)
        }

        Peer::Channel(channel) => {
            let id = channel.bare_id();

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

pub fn normalize_dialogs_into_data(dialogs: Vec<Dialog>) -> Arc<dashmap::DashMap<i64, DialogData>> {
    let dialogs_data: Arc<dashmap::DashMap<i64, DialogData>> = Arc::new(dashmap::DashMap::new());

    for dialog in dialogs {
        let peer = dialog.peer();
        let (id, dialog_data) = peer_to_dialog_data(peer);

        dialogs_data.insert(id, dialog_data);
    }

    dialogs_data
}
