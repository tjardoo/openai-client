use crate::v1::resources::chat_completion::{ChatCompletionParameters, ChatCompletionResponse};
use crate::v1::{api::Client, error::APIError};
use serde_json::Value;

pub struct Chat<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn chat(&self) -> Chat {
        Chat {
            client: self,
        }
    }
}

impl Chat<'_> {
    pub async fn create(&self, parameters: ChatCompletionParameters) -> Result<ChatCompletionResponse, APIError> {
        let response = self.client.post("/chat/completions", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let chat_completion_response: ChatCompletionResponse = serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(chat_completion_response)
    }
}
