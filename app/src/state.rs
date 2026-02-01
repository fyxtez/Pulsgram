use std::sync::Arc;

use telegram::dialogs::DialogData;

pub struct AppState {
    pub dialogs_data: Arc<dashmap::DashMap<i64, DialogData>>,
}
