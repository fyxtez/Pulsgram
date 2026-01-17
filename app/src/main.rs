use telegram::client::connect_client;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    println!("Hello, world!");

    let session_path = "plusgram.session";

    let _client = connect_client(session_path).await?;

    Ok(())
}
