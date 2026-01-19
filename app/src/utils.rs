use std::sync::Arc;

use grammers_client::{Client, types::Peer};
use telegram::dialogs::get_peer_by_bare_id;


pub async fn must_get_peer(
    client: &Arc<Client>,
    bare_id: i64,
    name: &str,
) -> Result<Peer, Box<dyn std::error::Error>> {
    get_peer_by_bare_id(client, bare_id)
        .await?
        .ok_or_else(|| format!("Could not find {} peer", name).into())
}
