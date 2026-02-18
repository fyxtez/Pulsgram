use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub chat_id: String,
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Chat>, sqlx::Error> {
    sqlx::query_as::<_, Chat>("SELECT * FROM chats")
        .fetch_all(pool)
        .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Chat>, sqlx::Error> {
    sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, name: &str, chat_id: &str) -> Result<Chat, sqlx::Error> {
    sqlx::query_as::<_, Chat>("INSERT INTO chats (name, chat_id) VALUES ($1, $2) RETURNING *")
        .bind(name)
        .bind(chat_id)
        .fetch_one(pool)
        .await
}
