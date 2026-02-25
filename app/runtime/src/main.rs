mod config;
mod error;
mod utils;

use api::start_api_server;
use std::sync::Arc;
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{build_peers_map_from_dialogs, load_dialogs, normalize_dialogs_into_data}, errors::TelegramError,
};

use crate::{config::Config, error::AppError, utils::create_reqwest_client};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Config::from_env(true)?;

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

    let dispatcher_me = telegram::get_me(&client_dispatcher).await?;
    let dispatcher_id = dispatcher_me.id().bare_id();
    
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

    let mut peers_map = build_peers_map_from_dialogs(&dialogs).await;
    let mut peers_map_dispatcher = build_peers_map_from_dialogs(&dialogs_from_dispatcher).await;

    drop(dialogs);
    drop(dialogs_from_dispatcher);

    let errors_peer = peers_map_dispatcher
        .remove(&config.errors_peer_id)
        .ok_or(AppError::NotFound("errors_peer_id"))?;

    let kol_follows = peers_map_dispatcher
        .remove(&config.kol_follows_chat_id)
        .ok_or(AppError::NotFound("kol_follows_chat_id"))?;

    let kol_follows_test = peers_map_dispatcher
        .remove(&config.kol_follows_test_chat_id)
        .ok_or(AppError::NotFound("kol_follows_test_chat_id"))?;

    let perp_signals = peers_map_dispatcher
        .remove(&config.perp_signals_chat_id)
        .ok_or(AppError::NotFound("perp_signals_chat_id"))?;

    let perp_signals_test = peers_map_dispatcher
        .remove(&config.perp_signals_test_chat_id)
        .ok_or(AppError::NotFound("perp_signals_test_chat_id"))?;

    let perp_kols = peers_map
        .remove(&config.perp_kols_chat_id)
        .ok_or(AppError::NotFound("perp_kols_chat_id"))?;

    let perp_kols_test = peers_map
        .remove(&config.perp_kols_test_chat_id)
        .ok_or(AppError::NotFound("perp_kols_test_chat_id"))?;

    let perp_kols_usernames: Vec<String> = config.perp_kols_usernames;

    drop(peers_map);
    drop(peers_map_dispatcher);


    let fyxtez_peer_ref = telegram::resolve_username(&client, "fyxtez")
        .await?
        .to_ref()
        .await
        .ok_or(AppError::Telegram(TelegramError::Other(String::from("Could not convert to PeerRef"))))?;

    let reqwest_client = create_reqwest_client()?;

    let _binance = binance::client::BinanceClient::new(
        reqwest_client.clone(),
        binance::constants::TESTNET_FUTURES,
        &config.binance_api_key,
        &config.binance_api_secret,
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
        config.lcs_user_id,
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
        config.rs_user_id,
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
        fyxtez_peer_ref,
        config.rs_user_id,
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
