use crate::v1::error::APIError;
#[cfg(feature = "stream")]
use crate::v1::resources::chat::ChatCompletionChunkResponse;
use crate::v1::resources::chat::{ChatCompletionParameters, ChatCompletionResponse};
use crate::v1::{api::Client, helpers::format_response};
#[cfg(feature = "stream")]
use futures::Stream;
#[cfg(feature = "stream")]
use std::pin::Pin;

pub struct Chat<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Given a list of messages comprising a conversation, the model will return a response.
    pub fn chat(&self) -> Chat {
        Chat { client: self }
    }
}

impl Chat<'_> {
    /// Creates a model response for the given chat conversation.
    pub async fn create(
        &self,
        parameters: ChatCompletionParameters,
    ) -> Result<ChatCompletionResponse, APIError> {
        let response = self.client.post("/chat/completions", &parameters).await?;

        let chat_completion_response: ChatCompletionResponse = format_response(response)?;

        Ok(chat_completion_response)
    }

    #[cfg(feature = "stream")]
    /// Creates a model response for the given chat conversation.
    pub async fn create_stream(
        &self,
        parameters: ChatCompletionParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionChunkResponse, APIError>> + Send>>,
        APIError,
    > {
        use crate::v1::resources::chat::StreamChatCompletionParameters;

        let stream_parameters = StreamChatCompletionParameters {
            messages: parameters.messages,
            model: parameters.model,
            frequency_penalty: parameters.frequency_penalty,
            logit_bias: parameters.logit_bias,
            max_tokens: parameters.max_tokens,
            n: parameters.n,
            presence_penalty: parameters.presence_penalty,
            response_format: parameters.response_format,
            stop: parameters.stop,
            stream: true,
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            tools: parameters.tools,
            tool_choice: parameters.tool_choice,
            user: parameters.user,
        };

        Ok(self
            .client
            .post_stream("/chat/completions", &stream_parameters)
            .await)
    }
}
