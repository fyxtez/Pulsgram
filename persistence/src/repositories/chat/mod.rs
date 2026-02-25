pub mod queries;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::error::PersistenceError;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub chat_id: String,
}

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Chat>, PersistenceError> {
        Ok(sqlx::query_as::<_, Chat>(queries::SELECT_ALL)
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Chat>, PersistenceError> {
        Ok(sqlx::query_as::<_, Chat>(queries::SELECT_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?)
    }

    pub async fn create(&self, name: &str, chat_id: &str) -> Result<Chat, PersistenceError> {
        Ok(sqlx::query_as::<_, Chat>(queries::INSERT)
            .bind(name)
            .bind(chat_id)
            .fetch_one(&self.pool)
            .await?)
    }
    pub async fn delete(&self, id: Uuid) -> Result<bool, PersistenceError> {
        let result = sqlx::query(queries::DELETE)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    #[ignore]
    async fn create_and_get_by_id(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        let created = repo
            .create("Test Chat", "tg_123")
            .await
            .expect("create failed");

        assert_eq!(created.name, "Test Chat");
        assert_eq!(created.chat_id, "tg_123");

        let fetched = repo
            .get_by_id(created.id)
            .await
            .expect("query failed")
            .expect("chat not found");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Test Chat");
    }

    #[sqlx::test]
    #[ignore]
    async fn create_duplicate_name_fails(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        repo.create("ChatA", "1").await.unwrap();

        let result = repo.create("ChatA", "2").await;

        assert!(result.is_err());
    }
    #[sqlx::test]
    #[ignore]
    async fn create_duplicate_chat_id_fails(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        repo.create("ChatA", "1").await.unwrap();

        let result = repo.create("ChatB", "1").await;

        assert!(result.is_err());
    }

    #[sqlx::test]
    #[ignore]
    async fn get_all_empty_returns_empty_vec(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        let all = repo.get_all().await.unwrap();
        assert!(all.is_empty());
    }

    #[sqlx::test]
    #[ignore]
    async fn get_all_returns_inserted_rows(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        repo.create("Chat1", "1").await.unwrap();
        repo.create("Chat2", "2").await.unwrap();

        let all = repo.get_all().await.unwrap();

        assert_eq!(all.len(), 2);
    }

    #[sqlx::test]
    #[ignore]
    async fn delete_removes_chat(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        let chat = repo.create("To Delete", "999").await.unwrap();

        let deleted = repo.delete(chat.id).await.unwrap();
        assert!(deleted);

        let after = repo.get_by_id(chat.id).await.unwrap();
        assert!(after.is_none());
    }

    #[sqlx::test]
    #[ignore]
    async fn delete_nonexistent_returns_false(pool: PgPool) {
        let repo = ChatRepository::new(pool);

        let random_id = Uuid::new_v4();
        let deleted = repo.delete(random_id).await.unwrap();

        assert!(!deleted);
    }
}
