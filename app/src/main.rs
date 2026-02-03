mod utils;

use api::start_api_server;
// use db::{connect, run_migrations};
use dotenv::dotenv;
use std::{collections::HashSet, sync::Arc, vec};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{
        DialogData, get_peer, load_dialogs, normalize_dialogs_into_data,
    },
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

    //TODO: this should be in a client!!!

    println!("Telegram dialogs loaded.");

    // TODO: Use function get_peers_by_bare_ids if you leaving it like this, and have endpoint to create them and save in DB.
    let _from_peer = get_peer(&client, 1649642332).await?;
    let _to_peer = get_peer(&client, 5173657056).await?;
    let _tokens_peer = get_peer(&client, 5144995821).await?;
    let errors_peer = get_peer(&client, 3876244916).await?;
    let _chartunan = get_peer(&client, 7690346837).await?;
    let users_group_peer = get_peer(&client, 3692507348).await?;

    println!("Peers fetched.");

    // TODO
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

    let chartuman_dialog_data = shared_state.dialogs_data.get(&7690346837).unwrap().clone();

    let r_bare_id = 6314293988;

    let targeted_users: Vec<DialogData> = vec![chartuman_dialog_data];
    let targeted_group_users: Vec<i64> = vec![r_bare_id, 7690346837];
    let targeted_channels:Vec<i64> = vec![3858893733];

    //TODO: Many of these arguments should go trough state.
    tokio::spawn(analysis::run(
        Arc::clone(&client),
        Arc::clone(&bus),
        shared_state.dialogs_data.clone(),
        errors_peer.clone(),
        users_group_peer,
        targeted_users,
        targeted_group_users,
        targeted_channels
    ));

    // let _db = connect("postgres://pulsgram_user:pulsgram_user@localhost:5432/pulsgram_db").await.unwrap();

    // run_migrations("../migrations", db);

    start_api_server("127.0.0.1", 8181, shared_state).await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
