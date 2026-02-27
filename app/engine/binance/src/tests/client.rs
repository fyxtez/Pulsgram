#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::types::OrderSide;
    use crate::utils::build_query;
    use crate::{client::BinanceClient, constants, error::BinanceError};

    fn test_client(url: &str) -> BinanceClient {
        dotenv::from_filename("app/.env").ok();

        let api_key = std::env::var("BINANCE_API_KEY_TEST").expect("Set BINANCE_API_KEY_TEST");

        let api_secret =
            std::env::var("BINANCE_API_SECRET_TEST").expect("Set BINANCE_API_SECRET_TEST");

        BinanceClient::new(reqwest::Client::new(), url, &api_key, &api_secret)
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_listener_key() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.create_listen_key().await;

        assert!(result.is_ok());

        let listen_key = result.unwrap();

        let close_result = client.close_listen_key(&listen_key).await;

        assert!(close_result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_get_account_info() {
        let client = test_client(constants::TESTNET_FUTURES);

        let account = client
            .get_account_info()
            .await
            .expect("Expected account info response");

        assert!(!account.assets.is_empty());

        assert!(
            account.assets.iter().any(|a| a.asset == "USDT"),
            "USDT asset not found in account"
        );
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_account_info() {
        let client = test_client(constants::TESTNET_FUTURES);

        let account = client.get_account_info().await.unwrap();

        assert!(account.total_wallet_balance.parse::<f64>().unwrap() >= 0.0);
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_get_trading_fees() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("BTCUSDT").await;

        assert!(result.is_ok());

        let fees = result.unwrap();

        assert_eq!(fees.symbol, "BTCUSDT");
        assert!(!fees.maker_commission_rate.is_empty());
        assert!(!fees.taker_commission_rate.is_empty());
        assert!(!fees.rpi_commission_rate.is_empty());
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_trading_fees_invalid_symbol() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("INVALIDSYMBOL").await;

        assert!(result.is_err());

        match result {
            Err(BinanceError::Api(msg)) => {
                assert!(msg.contains("Invalid symbol"));
            }
            Err(e) => panic!("Unexpected error variant: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_place_market_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        let order = client
            .place_market_order("BTCUSDT", &OrderSide::Buy, "0.01")
            .await
            .expect("market buy failed");

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.r#type, "MARKET");
        assert!(order.order_id > 0);

        // Close position (reverse order)
        client
            .place_market_order("BTCUSDT", &OrderSide::Sell, "0.01")
            .await
            .expect("market sell failed");
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_set_leverage() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.set_leverage("BTCUSDT", 5).await;

        match result {
            Ok(response) => {
                assert_eq!(response.symbol, "BTCUSDT");
                assert_eq!(response.leverage, 5);
            }

            Err(BinanceError::Api(msg)) => {
                println!("Leverage change failed: {}", msg);

                assert!(
                    msg.contains("-1000"),
                    "Unexpected Binance API error: {}",
                    msg
                );
            }

            Err(e) => panic!("Unexpected error variant: {:?}", e),
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_place_and_cancel_limit_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        // Place limit order far below market so it remains NEW
        let order = client
            .place_limit_order("BTCUSDT", &OrderSide::Buy, "0.01", "35000")
            .await
            .expect("failed to place limit order");

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.r#type, "LIMIT");
        assert!(order.order_id > 0);

        // For far-away price, order should normally remain NEW
        assert!(
            order.status == "NEW" || order.status == "FILLED",
            "unexpected order status: {}",
            order.status
        );

        let cancel = client
            .cancel_order("BTCUSDT", order.order_id)
            .await
            .expect("failed to cancel order");

        assert_eq!(cancel.status, "CANCELED");
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_get_open_orders() {
        let client = test_client(constants::TESTNET_FUTURES);

        let orders = client
            .get_open_orders(Some("BTCUSDT"))
            .await
            .expect("failed to fetch open orders");

        // Orders may or may not exist.
        // Just ensure request works and structure is valid.
        for order in &orders {
            assert_eq!(order.symbol, "BTCUSDT");
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_set_position_mode() {
        let client = test_client(constants::TESTNET_FUTURES);

        // Get current mode
        let current = client
            .get_position_mode()
            .await
            .expect("failed to fetch position mode");

        // If currently hedge mode, switch to one-way
        if current.dual_side_position {
            let result = client
                .set_position_mode(false)
                .await
                .expect("failed to change position mode");

            assert_eq!(result.dual_side_position, false);
        }

        // Verify final state
        let current = client
            .get_position_mode()
            .await
            .expect("failed to fetch position mode");

        assert_eq!(current.dual_side_position, false);
    }
    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_get_position_risk() {
        let client = test_client(constants::TESTNET_FUTURES);

        let positions = client
            .get_position_risk(Some("BTCUSDT"))
            .await
            .expect("failed to fetch position risk");

        // Request must succeed
        // Positions may be empty
        for pos in &positions {
            assert_eq!(pos.symbol, "BTCUSDT");
        }
    }

    #[tokio::test]
    #[ignore]
    #[serial(binance)]
    async fn test_invalid_quantity() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client
            .place_market_order("BTCUSDT", &OrderSide::Buy, "invalid")
            .await;

        assert!(result.is_err());
    }


    #[test]
    fn test_build_query_multiple_params() {
        let query = build_query(&[
            ("symbol", "BTCUSDT".to_string()),
            ("side", "BUY".to_string()),
        ]);

        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("side=BUY"));
        assert!(query.contains("&"));
    }
}
