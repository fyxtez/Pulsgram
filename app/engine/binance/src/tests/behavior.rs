#[cfg(test)]
mod integration_trade_flow {
    use serial_test::serial;
    use tokio::time::{Duration, sleep};

    use crate::types::OrderSide;
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

    async fn assert_flat_position(client: &BinanceClient, symbol: &str) {
        let positions = client
            .get_position_risk(Some(symbol))
            .await
            .expect("failed to fetch position risk");

        // If Binance returns empty list → account is flat
        if positions.is_empty() {
            return;
        }

        // If symbol entry exists, ensure size is zero
        if let Some(pos) = positions.into_iter().find(|p| p.symbol == symbol) {
            let amt: f64 = pos.position_amt.parse().unwrap_or(0.0);
            assert_eq!(amt, 0.0, "Position not flat");
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_open_wait_close() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";
        let qty = "0.01";

        cleanup_position(&client, symbol).await;

        // Open
        let open = client
            .place_market_order(symbol, &OrderSide::Buy, qty)
            .await
            .expect("failed to open");

        assert_eq!(open.symbol, symbol);
        assert_eq!(open.side, "BUY");

        sleep(Duration::from_secs(5)).await;

        // Close
        let close = client
            .place_market_order(symbol, &OrderSide::Sell, qty)
            .await
            .expect("failed to close");

        assert_eq!(close.side, "SELL");

        assert_flat_position(&client, symbol).await;
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_double_open_partial_closes() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";

        let full_qty = 0.02_f64;
        let half_qty = 0.02_f64;

        cleanup_position(&client, symbol).await;

        client
            .place_market_order(symbol, &OrderSide::Buy, &full_qty.to_string())
            .await
            .expect("first open failed");

        sleep(Duration::from_secs(2)).await;

        client
            .place_market_order(symbol, &OrderSide::Buy, &full_qty.to_string())
            .await
            .expect("second open failed");

        sleep(Duration::from_secs(2)).await;

        // First partial close
        client
            .place_market_order(symbol, &OrderSide::Sell, &half_qty.to_string())
            .await
            .expect("first partial close failed");

        sleep(Duration::from_secs(2)).await;

        // Second partial close
        client
            .place_market_order(symbol, &OrderSide::Sell, &half_qty.to_string())
            .await
            .expect("second partial close failed");

        assert_flat_position(&client, symbol).await;
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_accumulate_position() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";

        cleanup_position(&client, symbol).await;

        client
            .place_market_order(symbol, &OrderSide::Buy, "0.01")
            .await
            .unwrap();
        sleep(Duration::from_secs(2)).await;

        client
            .place_market_order(symbol, &OrderSide::Buy, "0.02")
            .await
            .unwrap();
        sleep(Duration::from_secs(2)).await;

        let positions = client.get_position_risk(Some(symbol)).await.unwrap();

        let position = positions
            .into_iter()
            .find(|p| p.symbol == symbol)
            .expect("Position not found");

        let position_amt: f64 = position.position_amt.parse().unwrap();

        assert_eq!(position_amt, 0.03_f64);
    }
    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    //From long 0.02 to short 0.03
    async fn test_flip_position() {
        let client = test_client(constants::TESTNET_FUTURES);
        let symbol = "BTCUSDT";

        cleanup_position(&client, symbol).await;

        client
            .place_market_order(symbol, &OrderSide::Buy, "0.02")
            .await
            .unwrap();
        sleep(Duration::from_secs(2)).await;

        // Sell more than long size
        client
            .place_market_order(symbol, &OrderSide::Sell, "0.03")
            .await
            .unwrap();
        sleep(Duration::from_secs(2)).await;

        let positions = client.get_position_risk(Some(symbol)).await.unwrap();

        let position = positions
            .into_iter()
            .find(|p| p.symbol == symbol)
            .expect("Position not found");

        let position_amt: f64 = position.position_amt.parse().unwrap();

        // Should now be short 0.01
        assert_eq!(position_amt, -0.01_f64);
    }
}

// TODO; Test zatvori poziciju tipa 50%
