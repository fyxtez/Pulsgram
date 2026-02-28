use std::sync::Arc;

use telegram::dialogs::DialogData;
use telegram_types::{Client, UpdatesLike};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::bootstrap::WorkersConfig;

pub struct TelegramRuntime {
    pub client: Arc<Client>,
    pub dispatcher: Arc<Client>,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
    pub dispatcher_id: i64,
    pub workers: WorkersConfig,
    pub dialogs_data: dashmap::DashMap<i64, DialogData>
}