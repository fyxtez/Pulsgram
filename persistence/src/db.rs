use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::time::timeout;
use sqlx::Error;
use axum::{response::{IntoResponse, Response}, http::StatusCode};

use crate::chats;

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

// TODO:
// match some_db_call.await {
//     Ok(val) => Ok(Json(val)),
//     Err(err) => Err(map_db_error(err)),
// }
pub fn map_db_error(err: Error) -> Response {
    println!("DB Error: {}", err);

    match err {
        Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            "Resource not found",
        )
            .into_response(),

        Error::Database(db_err) if db_err.code() == Some("23505".into()) => (
            StatusCode::CONFLICT,
            "Duplicate entry",
        )
            .into_response(),

        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
        )
            .into_response(),
    }
}

// TODO: This approach (JSON dump/restore) is a quick solution but not production-grade.
// The proper way to backup/restore a Postgres database is using pg_dump/psql:
//
// Backup:  pg_dump -U pulsgram_user -h localhost pulsgram > backup.sql
// Restore: psql -U pulsgram_user -h localhost pulsgram < backup.sql
//
// pg_dump handles schema, data, indexes, constraints and is far more reliable.
// Consider not using this code and using pg_dump in production.

#[allow(dead_code)]
pub async fn dump_all(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let chats = chats::get_all(pool).await?;

    let dump = json!({
        "chats": chats,
    });

    std::fs::write("db_dump.json", serde_json::to_string_pretty(&dump)?)?;
    println!("Dumped all data to db_dump.json");
    Ok(())
}
#[allow(dead_code)]
pub async fn restore_all(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("db_dump.json")?;
    let dump: serde_json::Value = serde_json::from_str(&json)?;

    if let Some(chats) = dump["chats"].as_array() {
        for chat in chats {
            chats::create(
                pool,
                chat["name"]
                    .as_str()
                    .ok_or("Invalid db_dump.json: chat.name missing")?,
                chat["chat_id"]
                    .as_str()
                    .ok_or("Invalid db_dump.json: chat.chat_id missing")?,
            )
            .await?;
        }
        println!("Restored chats");
    }

    Ok(())
}
