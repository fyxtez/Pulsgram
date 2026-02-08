mod utils;
mod constants;

use api::start_api_server;
// use db::{connect, run_migrations};
use dotenv::dotenv;
use std::{collections::HashSet, sync::Arc};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{build_peers_map_from_dialogs, get_peer, load_dialogs, normalize_dialogs_into_data},
};
use telegram_types::Peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_path = "plusgram.session";

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client(session_path).await?;

    println!("Telegram Client connected.");

    let client = Arc::new(client);

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    println!("Updates handler spawned.");

    let dialogs = load_dialogs(&client).await?;

    println!("Telegram dialogs loaded.");
    let peers_map = build_peers_map_from_dialogs(&dialogs);
    println!("Peers map l.");

    // let from_peer = peers_map
    //     .get(&1649642332)
    //     .ok_or("Could not find from_peer")?;
    // let to_peer = peers_map.get(&5173657056).ok_or("Could not find to_peer")?;
    // let tokens_peer = peers_map
    //     .get(&5144995821)
    //     .ok_or("Could not find tokens_peer")?;
    // let errors_peer = peers_map
    //     .get(&3876244916)
    //     .ok_or("Could not find errors_peer")?;
    // let users_group_peer = peers_map
    //     .get(&3692507348)
    //     .ok_or("Could not find users_group_peer")?;
    let kol_follows = peers_map
        .get(&3839014502)
        .ok_or("Could not find kol_follows")?;

    let lc_signals = peers_map
        .get(&5017001940)
        .ok_or("Could not find kol_follows")?;

    let fyxtez = client.resolve_username("Fyxtez").await?.unwrap();

    println!("Peers fetched.");

    let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

    let _ignored_peers: HashSet<&Peer> = HashSet::new();

    let dialogs_data = normalize_dialogs_into_data(dialogs);

    dbg!(&dialogs_data);

    println!("Dialogs normalized");

    // TODO: Ignore Ph & Ri

    // dump_dialogs_to_json(&dialogs_data, "dialogs.json").unwrap();

    let state = app_state::AppState { dialogs_data };

    let shared_state = Arc::new(state);

    // tokio::spawn(forwarder::run(
    //     Arc::clone(&client),
    //     from_peer,
    //     to_peer,
    //     Arc::clone(&bus),
    // ));

    // tokio::spawn(token_addressess_forwarder::run(
    //     Arc::clone(&bus),
    //     Arc::clone(&client),
    //     tokens_peer,
    //     _ignored_senders,
    //     _ignored_peers,
    // ));

    let kol_follows = kol_follows.clone();
    let lc_signals = lc_signals.clone();
    let targeted_kols: Vec<String> = vec![];

    tokio::spawn(lc_signals::run(
        Arc::clone(&bus),
        Arc::clone(&client),
        8084912410,
        lc_signals,
    ));


    tokio::spawn(kol_follows::run(
        Arc::clone(&bus),
        Arc::clone(&client),
        7910357312, //fyxtez t. bot
        targeted_kols,
        kol_follows,
        fyxtez,
    ));

    // let _db = connect("postgres://pulsgram_user:pulsgram_user@localhost:5432/pulsgram_db").await.unwrap();

    // run_migrations("../migrations", db);

    // start_api_server("127.0.0.1", 8181, shared_state).await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
