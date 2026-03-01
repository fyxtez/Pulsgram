use std::{sync::Arc, time::Duration};

use crate::{
    config::Config,
    error::AppError,
    types::TelegramRuntime,
    utils::{create_reqwest_client, get_build_version},
};
use api::start_api_server;
use app_state::AppState;
use binance::{client::BinanceClient, filters::extract_supported_filters};
use domain::types::symbol::Symbol;
use publisher::EventBus;
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{build_peers_map_from_dialogs, load_dialogs, normalize_dialogs_into_data},
};
use telegram_types::{Client, PeerRef, UpdatesLike};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct WorkersConfig {
    pub errors_peer: PeerRef,

    pub kol_follows_prod: PeerRef,
    pub kol_follows_test: PeerRef,

    pub perp_signals_prod: PeerRef,
    pub perp_signals_test: PeerRef,

    pub perp_kols_prod: PeerRef,
    pub perp_kols_test: PeerRef,

    pub perp_kols_usernames: Vec<String>,

    pub rs_user_id: i64,
    pub lcs_user_id: i64,
}

pub struct AppRuntime {
    pub state: Arc<AppState>,
    pub bus: Arc<EventBus>,
    pub client: Arc<Client>,
    pub client_dispatcher: Arc<Client>,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
    dispatcher_id: i64,
    pub workers: WorkersConfig,
    pub binance_client: Arc<BinanceClient>,
    pub listen_key: String,
}

pub async fn bootstrap() -> Result<AppRuntime, AppError> {
    let build_version = get_build_version();
    println!("Build Version: {}", build_version);

    let config = Config::from_env(true)?;

    let telegram = init_telegram(&config).await?;

    let reqwest_client = create_reqwest_client()?;

    let mut binance_client = binance::client::BinanceClient::new(
        reqwest_client.clone(),
        binance::constants::TESTNET_FUTURES,
        &config.binance_api_key,
        &config.binance_api_secret,
    );

    // TODO: Move exchangeInfo fetch + filter extraction into BinanceClient::build()
    // This is exchange-specific initialization logic and does not belong in bootstrap.
    let exchange_info = binance_client
        .get_exchange_info()
        .await
        .map_err(AppError::from)?;

    // TODO: Move SUPPORTED symbols into domain layer (domain::symbols::SUPPORTED_SYMBOLS)
    // Bootstrap should not define trading universe.
    let supported_symbols = vec![
        Symbol::BTC,
        Symbol::ETH,
        Symbol::SOL,
        Symbol::BNB,
        Symbol::XRP,
        Symbol::TRX,
        Symbol::ADA,
        Symbol::ASTER,
    ];

    let filters = extract_supported_filters(&exchange_info, &supported_symbols)?;
    binance_client.set_symbol_filters(filters);

    let bus = Arc::new(publisher::new_event_bus());

    let state = app_state::AppState {
        dialogs_data: telegram.dialogs_data,
        client: telegram.client.clone(),
        client_dispatcher: telegram.dispatcher.clone(),
        reqwest_client: reqwest_client.clone(),
        bus: bus.clone(),
    };

    let shared_state = Arc::new(state);

    let listen_key = binance_client.create_listen_key().await?;

    Ok(AppRuntime {
        state: shared_state,
        bus,
        client: telegram.client,
        client_dispatcher: telegram.dispatcher,
        dispatcher_id: telegram.dispatcher_id,
        updates_receiver: telegram.updates_receiver,
        workers: WorkersConfig {
            errors_peer: telegram.workers.errors_peer,

            kol_follows_prod: telegram.workers.kol_follows_prod,
            kol_follows_test: telegram.workers.kol_follows_test,

            perp_signals_prod: telegram.workers.perp_signals_prod,
            perp_signals_test: telegram.workers.perp_signals_test,

            perp_kols_prod: telegram.workers.perp_kols_prod,
            perp_kols_test: telegram.workers.perp_kols_test,

            perp_kols_usernames: telegram.workers.perp_kols_usernames,

            rs_user_id: config.rs_user_id,
            lcs_user_id: config.lcs_user_id,
        },
        binance_client: Arc::new(binance_client),
        listen_key,
    })
}

pub async fn init_telegram(config: &Config) -> Result<TelegramRuntime, AppError> {
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

    let dispatcher_me = telegram::get_me(&client_dispatcher).await?;
    let dispatcher_id = dispatcher_me.id().bare_id();

    let dialogs = load_dialogs(&client).await?;
    let dialogs_data = normalize_dialogs_into_data(&dialogs);

    if !cfg!(feature = "production") {
        dbg!(&dialogs_data);
    }

    let dialogs_from_dispatcher = load_dialogs(&client_dispatcher).await?;
    let dialogs_data_from_dispatcher = normalize_dialogs_into_data(&dialogs_from_dispatcher);

    if !cfg!(feature = "production") {
        dbg!(&dialogs_data_from_dispatcher);
    }

    let mut peers_map_dispatcher = build_peers_map_from_dialogs(&dialogs_from_dispatcher).await;

    let workers = WorkersConfig {
        errors_peer: peers_map_dispatcher
            .remove(&config.errors_peer_id)
            .ok_or(AppError::NotFound("errors_peer_id"))?,

        kol_follows_prod: peers_map_dispatcher
            .remove(&config.kol_follows_chat_id)
            .ok_or(AppError::NotFound("kol_follows_chat_id"))?,

        kol_follows_test: peers_map_dispatcher
            .remove(&config.kol_follows_test_chat_id)
            .ok_or(AppError::NotFound("kol_follows_test_chat_id"))?,

        perp_signals_prod: peers_map_dispatcher
            .remove(&config.perp_signals_chat_id)
            .ok_or(AppError::NotFound("perp_signals_chat_id"))?,

        perp_signals_test: peers_map_dispatcher
            .remove(&config.perp_signals_test_chat_id)
            .ok_or(AppError::NotFound("perp_signals_test_chat_id"))?,

        perp_kols_prod: peers_map_dispatcher
            .remove(&config.perp_kols_chat_id)
            .ok_or(AppError::NotFound("perp_kols_chat_id"))?,

        perp_kols_test: peers_map_dispatcher
            .remove(&config.perp_kols_test_chat_id)
            .ok_or(AppError::NotFound("perp_kols_test_chat_id"))?,

        perp_kols_usernames: config.perp_kols_usernames.clone(),

        rs_user_id: config.rs_user_id,
        lcs_user_id: config.lcs_user_id,
    };

    Ok(TelegramRuntime {
        client: Arc::new(client),
        dispatcher: Arc::new(client_dispatcher),
        updates_receiver,
        dispatcher_id,
        workers,
        dialogs_data,
    })
}

pub async fn run(runtime: AppRuntime) -> Result<(), AppError> {
    tokio::spawn(handle_updates(
        Arc::clone(&runtime.client),
        runtime.updates_receiver,
        Arc::clone(&runtime.bus),
        runtime.dispatcher_id,
    ));

    tokio::spawn(perp_signals::run(
        Arc::clone(&runtime.bus),
        Arc::clone(&runtime.client_dispatcher),
        runtime.workers.lcs_user_id,
        if cfg!(feature = "production") {
            runtime.workers.perp_signals_prod
        } else {
            runtime.workers.perp_signals_test
        },
    ));

    tokio::spawn(errors_reporter::run(
        Arc::clone(&runtime.client_dispatcher),
        runtime.workers.errors_peer,
        Arc::clone(&runtime.bus),
    ));

    tokio::spawn(kol_follows::run(
        Arc::clone(&runtime.bus),
        runtime.workers.rs_user_id,
        if cfg!(feature = "production") {
            runtime.workers.kol_follows_prod
        } else {
            runtime.workers.kol_follows_test
        },
        Arc::clone(&runtime.client_dispatcher),
    ));

    tokio::spawn(perp_kols::run(
        Arc::clone(&runtime.bus),
        Arc::clone(&runtime.client_dispatcher),
        runtime.workers.rs_user_id,
        if cfg!(feature = "production") {
            runtime.workers.perp_kols_prod
        } else {
            runtime.workers.perp_kols_test
        },
        runtime.workers.perp_kols_usernames,
    ));

    tokio::spawn(trade_executor::run(
        runtime.bus.clone(),
        runtime.binance_client.clone(),
    ));

    let client = runtime.binance_client.clone();
    let listen_key = runtime.listen_key.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30 * 60));

        loop {
            interval.tick().await;

            match client.keepalive_listen_key(&listen_key).await {
                Ok(_) => {
                    #[cfg(not(feature = "production"))]
                    println!("[LISTEN_KEY] Keepalive refreshed successfully");
                }
                Err(e) => {
                    eprintln!("[LISTEN_KEY] Keepalive FAILED: {:?}", e);
                }
            }
        }
    });

    let address = if cfg!(feature = "production") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    start_api_server(address, 8181, runtime.state).await?;

    Ok(())
}
