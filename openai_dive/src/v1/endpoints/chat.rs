use crate::v1::error::APIError;
#[cfg(feature = "stream")]
use crate::v1::resources::chat::ChatCompletionChunkResponse;
use crate::v1::resources::chat::{ChatCompletionParameters, ChatCompletionResponse};
use crate::v1::resources::shared::ResponseWrapper;
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
        let wrapped_response = self.create_wrapped(parameters).await?;

        Ok(wrapped_response.data)
    }

    /// Creates a model response for the given chat conversation.
    pub async fn create_wrapped(
        &self,
        parameters: ChatCompletionParameters,
    ) -> Result<ResponseWrapper<ChatCompletionResponse>, APIError> {
        let response = self.client.post("/chat/completions", &parameters).await?;

        let data: ChatCompletionResponse = format_response(response.data)?;

        Ok(ResponseWrapper {
            data,
            headers: response.headers,
        })
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
        let mut stream_parameters = parameters;
        stream_parameters.stream = Some(true);

        Ok(self
            .client
            .post_stream("/chat/completions", &stream_parameters)
            .await)
    }
}
