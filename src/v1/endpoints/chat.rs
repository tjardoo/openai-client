use crate::v1::resources::chat_completion::{ChatCompletionParameters, ChatCompletionResponse};
use crate::v1::api::Client;
use crate::v1::error::APIError;
use serde_json::Value;

#[cfg(feature = "stream")]
use std::collections::HashMap;
#[cfg(feature = "stream")]
use std::pin::Pin;
#[cfg(feature = "stream")]
use crate::v1::resources::chat_completion_stream::ChatCompletionStreamResponse;
#[cfg(feature = "stream")]
use crate::v1::resources::chat_completion::ChatMessage;
#[cfg(feature = "stream")]
use crate::v1::resources::shared::StopToken;
#[cfg(feature = "stream")]
use futures::Stream;
#[cfg(feature = "stream")]
use serde::Serialize;
#[cfg(feature = "stream")]
use crate::v1::resources::chat_completion::{Function, FunctionCallConfig};

#[cfg(feature = "simple")]
use crate::v1::resources::chat_completion::SimpleChatCompletionParameters;

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

    #[deprecated(since = "0.2.8")]
    #[cfg(feature = "simple")]
    pub async fn create_simple(&self, parameters: SimpleChatCompletionParameters) -> Result<ChatCompletionResponse, APIError> {
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
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            n: parameters.n,
            stream: true,
            stop: parameters.stop,
            max_tokens: parameters.max_tokens,
            presence_penalty: parameters.presence_penalty,
            frequency_penalty: parameters.frequency_penalty,
            logit_bias: parameters.logit_bias,
            functions: parameters.functions,
            function_call: parameters.function_call,
        };

        Ok(self.client.post_stream("/chat/completions", &stream_parameters).await)
    }
}

#[cfg(feature = "stream")]
#[derive(Serialize, Debug)]
struct ChatCompletionStreamParameters {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallConfig>,
}
