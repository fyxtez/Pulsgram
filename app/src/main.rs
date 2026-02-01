mod state;
mod utils;

use api::start_api_server;
// use db::{connect, run_migrations};
use dotenv::dotenv;
use std::{collections::HashSet, sync::Arc};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{
        DialogData, DialogType, get_dialog_type, get_dialogs, get_peer_by_bare_id, peer_to_dialog_data, print_dialogs, print_peer_data
    },
};
use telegram_types::Peer;

use crate::utils::must_get_peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_path = "plusgram.session";

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client(session_path).await?;

    let client = Arc::new(client);

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    let dialogs = get_dialogs(&client).await?;

    let from_peer = must_get_peer(&client, 1649642332, "from").await?;
    let to_peer = must_get_peer(&client, 5173657056, "to").await?;
    let tokens_peer = must_get_peer(&client, 5144995821, "tokens").await?;

    // TODO
    let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

    let _ignored_peers: HashSet<&Peer> = HashSet::new();

    // dodaj neki sa chartuman i probaj dal ce da ti kupi
    // let approved_peer_ids: Vec<i64> = vec![2040892468, 2450649967, 2227629400];

        let dialogs_data = dashmap::DashMap::new();

    for dialog in dialogs {
        let peer = dialog.peer();
        let (id, dialog_data) = peer_to_dialog_data(peer);

        dialogs_data.insert(id, dialog_data);
    }

    let state = state::AppState { dialogs_data };

    let _shared_state = Arc::new(state);

    dbg!(&_shared_state.dialogs_data);

    // tokio::spawn(buyer::run(
    //     Arc::clone(&client),
    //     to_peer.clone(),
    //     Arc::clone(&bus),
    //     approved_peer_ids,
    // ));

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

    // tokio::spawn(groups_analysis::run(Arc::clone(&client), Arc::clone(&bus)));

    // let _db = connect("postgres://pulsgram_user:pulsgram_user@localhost:5432/pulsgram_db").await.unwrap();

    // run_migrations("../migrations", db);


    start_api_server().await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
