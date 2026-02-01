use grammers_client::Client;
use grammers_client::InvocationError;
use grammers_client::types::Dialog;
use grammers_client::types::Peer;

#[derive(Debug)]
#[repr(u8)]
pub enum DialogType {
    User = 0,
    Group = 1,
    Channel = 2,
}

#[derive(Debug)]
pub struct DialogData {
    pub name: String,
    pub username: Option<String>,
    pub kind: DialogType,
}


pub fn get_dialog_type(dialog: &Dialog) -> DialogType {
    match dialog.peer {
        Peer::User(_) => DialogType::User,
        Peer::Group(_) => DialogType::Group,
        Peer::Channel(_) => DialogType::Channel,
    }
}

pub async fn get_dialogs(client: &Client) -> Result<Vec<Dialog>, InvocationError> {
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
    match peer {
        Peer::User(user) => {
            let id = user.bare_id();

            let data = DialogData {
                name: user.first_name().unwrap_or("Unnamed user").to_string(),
                username: user.username().map(|u| u.to_string()),
                kind: DialogType::User,
            };

            (id, data)
        }

        Peer::Group(group) => {
            let id = group.id().bare_id();

            let data = DialogData {
                name: group.title().unwrap_or("Unnamed group").to_string(),
                username: None,
                kind: DialogType::Group,
            };

            (id, data)
        }

        Peer::Channel(channel) => {
            let id = channel.bare_id();

            let data = DialogData {
                name: channel.title().to_string(),
                username: channel.username().map(|u| u.to_string()),
                kind: DialogType::Channel,
            };

            (id, data)
        }
    }
}