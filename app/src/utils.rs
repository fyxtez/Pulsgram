use std::sync::Arc;

use dashmap::DashMap;
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};
use telegram::dialogs::get_peer_by_bare_id;
use telegram_types::{Client, Peer};

pub async fn get_peer(
    client: &Arc<Client>,
    bare_id: i64,
) -> Result<Peer, Box<dyn std::error::Error>> {
    get_peer_by_bare_id(client, bare_id)
        .await?
        .ok_or_else(|| format!("Could not find peer with ID: {}", bare_id).into())
}

pub fn dump_dialogs_to_json<K, V, P>(
    dialogs: &DashMap<K, V>,
    path: P,
) -> Result<(), Box<dyn std::error::Error>>
where
    // Must use Clone in order to be call collect() and ofc entry.key/value.
    K: Eq + std::hash::Hash + Clone + Serialize,
    V: Clone + Serialize,
    P: AsRef<Path>,
{
    // Take a snapshot (DashMap → HashMap) because Dashmap cannot be serialized/deserialized.
    let snapshot: HashMap<K, V> = dialogs
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect();

    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &snapshot)?;

    Ok(())
}
