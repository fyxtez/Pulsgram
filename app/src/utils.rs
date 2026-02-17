use dashmap::DashMap;
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

#[allow(dead_code)]
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

pub fn create_reqwest_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Pulsgram/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    Ok(client)
}
