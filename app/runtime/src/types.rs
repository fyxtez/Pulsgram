use std::sync::Arc;

use app_state::AppState;
use binance::client::BinanceClient;
use publisher::EventBus;
use telegram::dialogs::DialogData;
use telegram_types::{Client, PeerRef, UpdatesLike};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct TelegramRuntime {
    pub client: Arc<Client>,
    pub dispatcher: Arc<Client>,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
    pub dispatcher_id: i64,
    pub workers: WorkersConfig,
    pub dialogs_data: dashmap::DashMap<i64, DialogData>,
}

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
    pub dispatcher_id: i64,
    pub workers: WorkersConfig,
    pub binance_client: Arc<BinanceClient>,
    pub listen_key: String,
}
