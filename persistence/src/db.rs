use serde::{Deserialize, Serialize};

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::time::timeout;

use crate::repositories::Repositories;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(900))
        .connect(database_url)
        .await
}

// TODO: This is a health for db. Make sure u have health for api as well.
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    timeout(
        Duration::from_secs(2),
        sqlx::query("SELECT 1").execute(pool),
    )
    .await
    .map_err(|_| sqlx::Error::PoolTimedOut)??;

    Ok(())
}

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;

    println!("Migrations ran successfully!");
    Ok(())
}

// TODO: This approach (JSON dump/restore) is a quick solution but not production-grade.
// The proper way to backup/restore a Postgres database is using pg_dump/psql:
//
// Backup:  pg_dump -U pulsgram_user -h localhost pulsgram > backup.sql
// Restore: psql -U pulsgram_user -h localhost pulsgram < backup.sql
//
// pg_dump handles schema, data, indexes, constraints and is far more reliable.
// Consider not using this code and using pg_dump in production.

#[derive(Serialize, Deserialize)]
struct Dump {
    chats: Vec<crate::repositories::chat::Chat>,
}

#[allow(dead_code)]
pub async fn dump_all(repos: &Repositories) -> Result<(), Box<dyn std::error::Error>> {
    let dump = Dump {
        chats: repos.chat.get_all().await?,
    };

    std::fs::write("db_dump.json", serde_json::to_string_pretty(&dump)?)?;

    println!("Dumped all data to db_dump.json");
    Ok(())
}
#[allow(dead_code)]
pub async fn restore_all(repos: &Repositories) -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("db_dump.json")?;

    let dump: Dump = serde_json::from_str(&json)?;

    for chat in dump.chats {
        repos.chat.create(&chat.name, &chat.chat_id).await?;
    }

    println!("Restored chats");
    Ok(())
}
