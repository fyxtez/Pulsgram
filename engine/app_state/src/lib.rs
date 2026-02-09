use std::sync::Arc;

use telegram::dialogs::DialogData;
use telegram_types::Client;

pub struct AppState {
    pub dialogs_data: Arc<dashmap::DashMap<i64, DialogData>>,
    pub client: Arc<Client>,
    pub client_dispatcher: Arc<Client>,
}
