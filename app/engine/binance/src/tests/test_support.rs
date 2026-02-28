#[cfg(test)]
pub mod test_support {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    use domain::types::symbol::{Symbol, SymbolFilters};
    use reqwest;

    use crate::client::BinanceClient;

    static TEST_SYMBOL_FILTERS: OnceLock<HashMap<Symbol, SymbolFilters>> = OnceLock::new();

    fn filters() -> &'static HashMap<Symbol, SymbolFilters> {
        TEST_SYMBOL_FILTERS.get_or_init(|| {
            let mut map = HashMap::new();

            map.insert(
                Symbol::BTC,
                SymbolFilters {
                    step_size: 0.001,
                    min_qty: 0.001,
                    min_notional: 100.0,
                    tick_size: 0.1,
                },
            );

            map.insert(
                Symbol::ETH,
                SymbolFilters {
                    step_size: 0.001,
                    min_qty: 0.001,
                    min_notional: 20.0,
                    tick_size: 0.01,
                },
            );

            map.insert(
                Symbol::SOL,
                SymbolFilters {
                    step_size: 0.01,
                    min_qty: 0.01,
                    min_notional: 5.0,
                    tick_size: 0.01,
                },
            );

            map.insert(
                Symbol::BNB,
                SymbolFilters {
                    step_size: 0.01,
                    min_qty: 0.01,
                    min_notional: 5.0,
                    tick_size: 0.01,
                },
            );

            map.insert(
                Symbol::XRP,
                SymbolFilters {
                    step_size: 0.1,
                    min_qty: 0.1,
                    min_notional: 5.0,
                    tick_size: 0.0001,
                },
            );

            map.insert(
                Symbol::TRX,
                SymbolFilters {
                    step_size: 1.0,
                    min_qty: 1.0,
                    min_notional: 5.0,
                    tick_size: 0.00001,
                },
            );

            map.insert(
                Symbol::ADA,
                SymbolFilters {
                    step_size: 1.0,
                    min_qty: 1.0,
                    min_notional: 5.0,
                    tick_size: 0.0001,
                },
            );

            map.insert(
                Symbol::ASTER,
                SymbolFilters {
                    step_size: 1.0,
                    min_qty: 1.0,
                    min_notional: 5.0,
                    tick_size: 0.0001,
                },
            );

            map
        })
    }

    pub fn test_client(url: &str) -> BinanceClient {
        dotenv::from_filename("app/.env").ok();

        let api_key = std::env::var("BINANCE_API_KEY_TEST").expect("Set BINANCE_API_KEY_TEST");

        let api_secret =
            std::env::var("BINANCE_API_SECRET_TEST").expect("Set BINANCE_API_SECRET_TEST");

        let mut client = BinanceClient::new(reqwest::Client::new(), url, &api_key, &api_secret);

        client.set_symbol_filters(filters().clone());

        client
    }
}
