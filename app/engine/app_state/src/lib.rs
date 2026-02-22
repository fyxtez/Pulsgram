use std::sync::Arc;

use telegram::dialogs::DialogData;
use telegram_types::Client;

pub struct AppState {
    pub dialogs_data: dashmap::DashMap<i64, DialogData>,
    pub client: Arc<Client>,
    pub client_dispatcher: Arc<Client>,
    pub reqwest_client: reqwest::Client, // We can use reqwest::Client directly without wrapping it in Arc, since it's designed to be cloned and shared across threads.
}
