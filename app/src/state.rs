use telegram::dialogs::DialogData;


pub struct AppState {
    pub dialogs_data: dashmap::DashMap<i64,DialogData>,
}
