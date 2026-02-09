mod constants;
mod utils;

use api::start_api_server;
// use db::{connect, run_migrations};
use dotenv::dotenv;
use std::sync::Arc;
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{build_peers_map_from_dialogs, get_peer, load_dialogs, normalize_dialogs_into_data},
};
use telegram_types::Peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client("plusgram.session","API_ID","API_HASH","PHONE_NUMBER","PASSWORD").await?;

    let ConnectClientReturnType {
        client: dispatcher_client,
        updates_receiver: _,
    } = connect_client("dispatcher.plusgram.session","API_ID","API_HASH","PHONE_NUMBER_DISPATCHER","PASSWORD_DISPATCHER").await?;

    let client = Arc::new(client);

    let client_dispatcher = Arc::new(dispatcher_client);

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    let dialogs = load_dialogs(&client).await?;

    let dialogs_data = normalize_dialogs_into_data(&dialogs);

    // dbg!(&dialogs_data);

    let dialogs_from_dispatcher = load_dialogs(&client_dispatcher).await?;

    let dialogs_data_from_dispatcher = normalize_dialogs_into_data(&dialogs_from_dispatcher);

    dbg!(dialogs_data_from_dispatcher);

    let peers_map_dispatcher = build_peers_map_from_dialogs(&dialogs_from_dispatcher);

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
    let kol_follows = peers_map_dispatcher
        .get(&3839014502)
        .ok_or("Could not find kol_follows")?;

    let perp_signals = peers_map_dispatcher
        .get(&3725788750)
        .ok_or("Could not find lc_signals")?;

    // let fyxtez = client.resolve_username("Fyxtez").await?.unwrap();

    println!("Peers fetched.");

    // let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

    // let _ignored_peers: HashSet<&Peer> = HashSet::new();



    println!("Dialogs normalized");

    // TODO: Ignore Ph & Ri

    // dump_dialogs_to_json(&dialogs_data, "dialogs.json").unwrap();

    let state = app_state::AppState { dialogs_data,client:client.clone(),client_dispatcher:client_dispatcher.clone() };

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
    let lc_signals = perp_signals.clone();

    tokio::spawn(lc_signals::run(
        Arc::clone(&bus),
        Arc::clone(&client),
        8084912410,
        lc_signals,
    ));

    tokio::spawn(kol_follows::run(
        Arc::clone(&bus),
        7910357312,
        kol_follows,
        Arc::clone(&client_dispatcher),
    ));

    // let _db = connect("postgres://pulsgram_user:pulsgram_user@localhost:5432/pulsgram_db").await.unwrap();

    // run_migrations("../migrations", db);

    start_api_server("127.0.0.1", 8181, shared_state).await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
