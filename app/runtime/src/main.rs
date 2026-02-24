mod utils;

use api::start_api_server;
use dotenv::dotenv;
use std::{env, sync::Arc};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{build_peers_map_from_dialogs, load_dialogs, normalize_dialogs_into_data},
};

use crate::utils::create_reqwest_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client(
        "pulsgram.session",
        "API_ID",
        "API_HASH",
        "PHONE_NUMBER",
        "PASSWORD",
    )
    .await?;

    let ConnectClientReturnType {
        client: client_dispatcher,
        updates_receiver: _,
    } = connect_client(
        "dispatcher.pulsgram.session",
        "API_ID",
        "API_HASH",
        "PHONE_NUMBER_DISPATCHER",
        "PASSWORD_DISPATCHER",
    )
    .await?;

    let client = Arc::new(client);
    let client_dispatcher = Arc::new(client_dispatcher);

    let dispatcher_me = client_dispatcher.get_me().await?;
    let dispatcher_id = dispatcher_me.bare_id();

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
        dispatcher_id,
    ));

    let dialogs = load_dialogs(&client).await?;
    let dialogs_data = normalize_dialogs_into_data(&dialogs);
    if !cfg!(feature = "production") {
        dbg!(&dialogs_data);
    }

    let dialogs_from_dispatcher = load_dialogs(&client_dispatcher).await?;
    let dialogs_data_from_dispatcher = normalize_dialogs_into_data(&dialogs_from_dispatcher);
    if !cfg!(feature = "production") {
        dbg!(dialogs_data_from_dispatcher);
    }

    let mut peers_map = build_peers_map_from_dialogs(&dialogs);
    let mut peers_map_dispatcher = build_peers_map_from_dialogs(&dialogs_from_dispatcher);

    drop(dialogs);
    drop(dialogs_from_dispatcher);

    let kol_follows = peers_map_dispatcher
        .remove(&env::var("KOL_FOLLOWS_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Could not find kol_follows")?;

    let errors_peer = peers_map_dispatcher
        .remove(&env::var("ERRORS_PEER_ID")?.parse::<i64>()?)
        .ok_or("Could not find errors peer")?;

    let kol_follows_test = peers_map_dispatcher
        .remove(&env::var("KOL_FOLLOWS_TEST_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Cant find test kol follow")?;

    let perp_signals = peers_map_dispatcher
        .remove(&env::var("PERP_SIGNALS_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Could not find perp_signals")?;

    let perp_signals_test = peers_map_dispatcher
        .remove(&env::var("PERP_SIGNALS_TEST_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Could not find perp_signals_test")?;

    let perp_kols = peers_map
        .remove(&env::var("PERP_KOLS_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Could not find perp_kols")?;

    let perp_kols_test = peers_map
        .remove(&env::var("PERP_KOLS_TEST_CHAT_ID")?.parse::<i64>()?)
        .ok_or("Could not find perp_kols_test")?;

    let perp_kols_usernames: Vec<String> = env::var("PERP_KOLS_USERNAMES")?
        .split(',')
        .map(|s| s.to_string())
        .collect();

    drop(peers_map);
    drop(peers_map_dispatcher);

    let fyxtez = client
        .resolve_username("fyxtez")
        .await?
        .ok_or("Username fyxtez not found")?;

    let use_testnet = true;

    let (api_key_var, api_secret_var) = if use_testnet {
        ("BINANCE_API_KEY_TEST", "BINANCE_API_SECRET_TEST")
    } else {
        ("BINANCE_API_KEY", "BINANCE_API_SECRET")
    };
    let binance_env_vars = binance::utils::load_env_vars(api_key_var, api_secret_var)?;

    let reqwest_client = create_reqwest_client()?;

    let _binance = binance::client::BinanceClient::new(
        reqwest_client.clone(),
        binance::constants::TESTNET_FUTURES,
        &binance_env_vars.api_key,
        &binance_env_vars.api_secret,
    );

    let state = app_state::AppState {
        dialogs_data,
        client: client.clone(),
        client_dispatcher: client_dispatcher.clone(),
        reqwest_client,
    };

    let shared_state = Arc::new(state);

    tokio::spawn(perp_signals::run(
        Arc::clone(&bus),
        Arc::clone(&client_dispatcher),
        env::var("LCS_USER_ID")?.parse::<i64>()?,
        if cfg!(feature = "production") {
            perp_signals
        } else {
            perp_signals_test
        },
    ));

    tokio::spawn(errors_reporter::run(
        Arc::clone(&client_dispatcher),
        errors_peer,
        Arc::clone(&bus),
    ));

    tokio::spawn(kol_follows::run(
        Arc::clone(&bus),
        env::var("RS_USER_ID")?.parse::<i64>()?,
        if cfg!(feature = "production") {
            kol_follows
        } else {
            kol_follows_test
        },
        Arc::clone(&client_dispatcher),
    ));

    tokio::spawn(perp_kols::run(
        Arc::clone(&bus),
        // TODO: change this to dispatcher after wolfy answers the bot problem.
        Arc::clone(&client),
        fyxtez,
        env::var("RS_USER_ID")?.parse::<i64>()?,
        if cfg!(feature = "production") {
            perp_kols
        } else {
            perp_kols_test
        },
        perp_kols_usernames,
    ));

    let address = if cfg!(feature = "production") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    start_api_server(address, 8181, shared_state).await?;

    Ok(())
}
