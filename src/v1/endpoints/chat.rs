use crate::v1::resources::chat_completion::{ChatCompletionParameters, ChatCompletionResponse, ChatMessage};
use crate::v1::{api::Client, error::APIError};
use serde::Serialize;
use serde_json::Value;

#[cfg(feature = "stream")]
use std::pin::Pin;
#[cfg(feature = "stream")]
use crate::v1::resources::chat_completion_stream::ChatCompletionStreamResponse;
#[cfg(feature = "stream")]
use futures::Stream;

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

    #[cfg(feature = "stream")]
    pub async fn create_stream(&self, parameters: ChatCompletionParameters) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionStreamResponse, APIError>> + Send>>, APIError> {
        let stream_parameters = ChatCompletionStreamParameters {
            model: parameters.model,
            messages: parameters.messages,
            max_tokens: parameters.max_tokens,
            temperature: parameters.temperature,
            stream: true,
        };

        Ok(self.client.post_stream("/chat/completions", &stream_parameters).await)
    }
}

#[cfg(feature = "stream")]
#[derive(Serialize, Debug)]
struct ChatCompletionStreamParameters {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}
