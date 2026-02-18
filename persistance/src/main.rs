mod db;
mod queries;
use crate::{
    chats::{create, get_by_id},
    db::{connect, dump_all, health_check, run},
};
pub use queries::chats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect(&database_url).await;

    println!("Connected to database!");

    health_check(&pool).await?;

    run(&pool).await?;

    let chat = create(&pool, "Test", "123").await?;

    dump_all(&pool).await?;

    let all_chats = chats::get_all(&pool).await?;
    println!("Chats: {:?}", all_chats);

    let chat_by_id = get_by_id(&pool, chat.id).await?;
    println!("Chat by ID: {:?}", chat_by_id);

    Ok(())
}
