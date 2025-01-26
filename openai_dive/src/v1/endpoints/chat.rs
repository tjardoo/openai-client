use crate::v1::error::APIError;
#[cfg(feature = "stream")]
use crate::v1::resources::chat::ChatCompletionChunkResponse;
#[cfg(feature = "stream")]
use crate::v1::resources::chat::DeltaChatMessage;
use crate::v1::resources::chat::{ChatCompletionParameters, ChatCompletionResponse};
use crate::v1::resources::shared::ResponseWrapper;
use crate::v1::{api::Client, helpers::format_response};
#[cfg(feature = "stream")]
use futures::Stream;
#[cfg(feature = "stream")]
use std::pin::Pin;
#[cfg(feature = "stream")]
use std::task::{Context, Poll};

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

#[cfg(feature = "stream")]
enum CurrentRole {
    User,
    System,
    Assistant,
}

#[cfg(feature = "stream")]
pub struct RoleTrackingStream<S> {
    stream: S,
    current_role: Option<CurrentRole>,
}

#[cfg(feature = "stream")]
impl<S> RoleTrackingStream<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            current_role: None,
        }
    }
}

#[cfg(feature = "stream")]
impl<S> Stream for RoleTrackingStream<S>
where
    S: Stream<Item = Result<ChatCompletionChunkResponse, APIError>> + Unpin,
{
    type Item = Result<ChatCompletionChunkResponse, APIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        match Pin::new(&mut this.stream).poll_next(cx) {
            Poll::Ready(Some(Ok(mut chat_response))) => {
                chat_response.choices.iter_mut().for_each(|choice| {
                    match &choice.delta {
                        DeltaChatMessage::User { .. } => {
                            this.current_role = Some(CurrentRole::User)
                        }
                        DeltaChatMessage::System { .. } => {
                            this.current_role = Some(CurrentRole::System)
                        }
                        DeltaChatMessage::Assistant { .. } => {
                            this.current_role = Some(CurrentRole::Assistant)
                        }
                        _ => {}
                    }

                    if let DeltaChatMessage::Untagged {
                        content,
                        reasoning_content,
                        refusal,
                        name: _,
                        tool_calls,
                        tool_call_id: _,
                    } = &mut choice.delta
                    {
                        match this.current_role {
                            Some(CurrentRole::User) => {
                                choice.delta = DeltaChatMessage::User {
                                    name: Some("user".to_string()),
                                    content: content.clone().unwrap(),
                                }
                            }
                            Some(CurrentRole::System) => {
                                choice.delta = DeltaChatMessage::System {
                                    name: Some("system".to_string()),
                                    content: content.clone().unwrap(),
                                }
                            }
                            Some(CurrentRole::Assistant) => {
                                choice.delta = DeltaChatMessage::Assistant {
                                    name: Some("assistant".to_string()),
                                    content: content.clone(),
                                    reasoning_content: reasoning_content.clone(),
                                    refusal: refusal.clone(),
                                    tool_calls: tool_calls.clone(),
                                }
                            }
                            _ => {}
                        }
                    }
                });

                Poll::Ready(Some(Ok(chat_response)))
            }
            other => other,
        }
    }
}
