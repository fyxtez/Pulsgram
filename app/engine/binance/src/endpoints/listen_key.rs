use reqwest::Method;

use crate::{
    client::BinanceClient, endpoints::LISTEN_KEY, errors::BinanceError,
    response_types::ListenKeyResponse, utils::build_query,
};

impl BinanceClient {
    pub async fn create_listen_key(&self) -> Result<String, BinanceError> {
        let resp: ListenKeyResponse = self
            .transport()
            .api_key(Method::POST, LISTEN_KEY, None)
            .await?;

        Ok(resp.listen_key)
    }

    pub async fn close_listen_key(&self, listen_key: &str) -> Result<(), BinanceError> {
        let query = build_query(&[("listenKey", listen_key.to_string())]);

        let _: serde_json::Value = self
            .transport()
            .api_key(Method::DELETE, LISTEN_KEY, Some(query))
            .await?;

        Ok(())
    }

    pub async fn keepalive_listen_key(&self, listen_key: &str) -> Result<(), BinanceError> {
        let query = build_query(&[("listenKey", listen_key.to_string())]);

        let _: serde_json::Value = self
            .transport()
            .api_key(Method::PUT, LISTEN_KEY, Some(query))
            .await?;

        Ok(())
    }
}
