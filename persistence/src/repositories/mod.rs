pub mod chat;
pub mod error;

use sqlx::PgPool;

pub struct Repositories {
    pub chat: chat::ChatRepository,
}

impl Repositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            chat: chat::ChatRepository::new(pool.clone()),
        }
    }
}
