use reqwest_websocket::{RequestBuilderExt, WebSocket};

use crate::v1::{api::Client, error::APIError};

pub struct Realtime<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn realtime(&self) -> Realtime {
        Realtime { client: self }
    }
}

impl Realtime<'_> {
    pub async fn websocket(&self, model: &str) -> Result<WebSocket, APIError> {
        let response = reqwest::Client::default()
            .get(format!("wss://api.openai.com/v1/realtime?model={}", model))
            .header("OpenAI-Beta", "realtime=v1")
            .bearer_auth(&self.client.api_key)
            .upgrade()
            .send()
            .await?;

        let websocket = response.into_websocket().await?;

        Ok(websocket)
    }
}
