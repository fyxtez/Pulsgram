use grammers_client::Client;
use grammers_client::InvocationError;
use grammers_client::types::Dialog;
use grammers_client::types::Peer;

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

pub fn print_dialogs(dialogs: &Vec<Dialog>) -> Result<(), InvocationError> {

    // TODO: This must be written betterly with proper formatting
    for dialog in dialogs {
        let peer = dialog.peer();

        let (peer_type, id, title, username, extra) = match peer {
            Peer::Channel(c) => (
                "channel",
                c.bare_id().to_string(),
                c.title().to_string(),
                format!(
                    "admin_rights={:?} username={}",
                    c.admin_rights(),
                    c.username().unwrap_or("NONE USERNAME")
                ),
                "extra".to_string(),
            ),

            Peer::Group(c) => (
                "group",
                c.id().to_string(),
                c.is_megagroup().to_string(),
                c.username().unwrap_or("<no name>").to_string(),
                c.title().unwrap_or("No Title").to_string(),
            ),

            Peer::User(u) => (
                "user",
                u.bare_id().to_string(),
                u.first_name().unwrap_or("<no first name>").to_string(),
                u.username().unwrap_or("<no username>").to_string(),
                u.is_bot().to_string(),
            ),
        };

        println!(
            "Dialog | {:<7} | id: {:<12} | {:<30} | last_message: {:?} | {} | {}",
            peer_type, id, title, dialog.last_message, username, extra
        );
    }

    Ok(())
}
