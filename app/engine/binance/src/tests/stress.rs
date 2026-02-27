#[cfg(test)]
mod integration_trade_flow {
    use domain::types::order_side::OrderSide;
    use serial_test::serial;
    use tokio::time::{Duration, sleep};

    use crate::{client::BinanceClient, constants};

    fn test_client(url: &str) -> BinanceClient {
        dotenv::from_filename("app/.env").ok();

        let api_key = std::env::var("BINANCE_API_KEY_TEST").expect("Set BINANCE_API_KEY_TEST");

        let api_secret =
            std::env::var("BINANCE_API_SECRET_TEST").expect("Set BINANCE_API_SECRET_TEST");

        BinanceClient::new(reqwest::Client::new(), url, &api_key, &api_secret)
    }

    async fn cleanup_position(client: &BinanceClient, symbol: &str) {
        let positions = client
            .get_position_risk(Some(symbol))
            .await
            .expect("failed to fetch position risk");

        for pos in positions {
            if pos.symbol != symbol {
                continue;
            }

            let amt: f64 = pos.position_amt.parse().unwrap_or(0.0);

            // If position amount is positive:
            // This means there is an open LONG position.
            // To flatten it, submit a SELL market order
            // for the exact same quantity.
            if amt > 0.0 {
                client
                    .place_market_order(symbol, &OrderSide::Sell, &amt.to_string())
                    .await
                    .expect("failed to cleanup long position");
            }

            // If position amount is negative:
            // This means there is an open SHORT position.
            // To flatten it, submit a BUY market order
            // for the absolute value of the position size.
            if amt < 0.0 {
                client
                    .place_market_order(symbol, &OrderSide::Buy, &(-amt).to_string())
                    .await
                    .expect("failed to cleanup short position");
            }
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_rapid_sequential_orders() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";

        cleanup_position(&client, symbol).await;

        for _ in 0..5 {
            client
                .place_market_order(symbol, &OrderSide::Buy, "0.01")
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(3)).await;

        let positions = client.get_position_risk(Some(symbol)).await.unwrap();

        let pos = positions.into_iter().find(|p| p.symbol == symbol).unwrap();
        let amt: f64 = pos.position_amt.parse().unwrap();

        assert_eq!(amt, 0.05_f64);
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_rapid_sequential_orders_with_cleanup() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";

        cleanup_position(&client, symbol).await;

        for _ in 0..5 {
            client
                .place_market_order(symbol, &OrderSide::Buy, "0.01")
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(3)).await;

        let positions = client.get_position_risk(Some(symbol)).await.unwrap();

        let pos = positions.into_iter().find(|p| p.symbol == symbol).unwrap();
        let amt: f64 = pos.position_amt.parse().unwrap();

        assert_eq!(amt, 0.05_f64);

        // 🔥 cleanup
        cleanup_position(&client, symbol).await;
    }
}
