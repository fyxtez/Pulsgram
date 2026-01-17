use std::{
    error::Error,
    io::{self},
};

use grammers_client::{Client, SignInError};
use grammers_session::{storages::SqliteSession};
use grammers_mtsender::SenderPool;

use crate::config::load_tg_client_config;


pub async fn connect_client(
    session_path: &str,
) -> Result<Client, Box<dyn Error>> {
    let config = load_tg_client_config()?;

    let session = SqliteSession::open(session_path)?;


    let pool = SenderPool::new(std::sync::Arc::new(session), config.api_id);

    let client = Client::new(&pool);

      let SenderPool { runner, .. } = pool;
    let _ = tokio::spawn(runner.run());

    if !client.is_authorized().await? {
        let token = client.request_login_code(&config.phone_number, config.api_hash.as_str()).await?;

        println!("Enter the OTP code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, &config.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    //proveri dal se seija cuva i ne brise kad se napravi nova tj ulogujes se

    println!(
        "Connected to Telegram via {}!",
        client.get_me().await.unwrap().full_name()
    );
    Ok(client)
}
