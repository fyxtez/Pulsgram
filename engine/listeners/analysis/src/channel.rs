use std::sync::Arc;

use telegram::client::report_error;
use telegram_types::{Client, Message, Peer};

pub async fn handle(
    client:Arc<Client>,
    message: &Message,
    errors_peer:Peer
    
) {
    let peer = message.peer();

    if peer.is_err(){
        report_error(client, errors_peer, format!("Something went wrong fetching peer. Message: {}",message.text()));
    }
    let peer = peer.unwrap();

    let name = peer.name().unwrap_or("UNNAMED CHANNEL");

    dbg!(name);

}
