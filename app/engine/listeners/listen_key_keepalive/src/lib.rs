use std::{sync::Arc, time::Duration};

use binance::client::BinanceClient;

pub async fn run(client: Arc<BinanceClient>, listen_key: String) {
    let mut interval = tokio::time::interval(Duration::from_secs(30 * 60));

    loop {
        interval.tick().await;

        match client.keepalive_listen_key(&listen_key).await {
            Ok(_) => {
                #[cfg(not(feature = "production"))]
                println!("[LISTEN_KEY] Keepalive refreshed successfully");
            }
            Err(e) => {
                eprintln!("[LISTEN_KEY] Keepalive FAILED: {:?}", e);
            }
        }
    }
}
