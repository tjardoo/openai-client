use serde::{Deserialize, Serialize};

use super::shared::{
    Content, Conversation, ErrorDetail, Item, RateLimit, Response, Session, TranscriptionError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "error".
    pub r#type: String,
    /// Details of the error.
    pub error: ErrorDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "session.created".
    pub r#type: String,
    /// The session resource.
    pub session: Session,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "session.updated".
    pub r#type: String,
    /// The updated session resource.
    pub session: Session,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.created".
    pub r#type: String,
    /// The conversation resource.
    pub conversation: Conversation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "input_audio_buffer.committed".
    pub r#type: String,
    /// The ID of the preceding item after which the new item will be inserted.
    pub previous_item_id: String,
    /// The ID of the user message item that will be created.
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "input_audio_buffer.cleared".
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputAudioBufferSpeechStarted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "input_audio_buffer.speech_started".
    pub r#type: String,
    /// Milliseconds since the session started when speech was detected.
    pub audio_start_ms: u32,
    /// The ID of the user message item that will be created when speech stops.
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputAudioBufferSpeechStopped {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "input_audio_buffer.speech_stopped".
    pub r#type: String,
    /// Milliseconds since the session started when speech stopped.
    pub audio_start_ms: u32,
    /// The ID of the user message item that will be created.
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationItemCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.item.created".
    pub r#type: String,
    /// The ID of the preceding item.
    pub previous_item_id: String,
    /// The item that was created.
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationItemInputAudioTranscriptionCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.item.input_audio_transcription.completed".
    pub r#type: String,
    /// The ID of the user message item.
    pub item_id: String,
    /// The index of the content part containing the audio.
    pub content_index: u32,
    /// The transcribed text.
    pub transcript: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationItemInputAudioTranscriptionFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.item.input_audio_transcription.failed".
    pub r#type: String,
    /// The ID of the user message item.
    pub item_id: String,
    /// The index of the content part containing the audio.
    pub content_index: u32,
    /// Details of the transcription error.
    pub error: TranscriptionError,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationItemTruncated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.item.truncated".
    pub r#type: String,
    /// The ID of the assistant message item that was truncated.
    pub item_id: String,
    /// The index of the content part that was truncated.
    pub content_index: u32,
    /// The duration up to which the audio was truncated, in milliseconds.
    pub audio_end_ms: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConversationItemDeleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "conversation.item.deleted".
    pub r#type: String,
    /// The ID of the item that was deleted.
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.created".
    pub r#type: String,
    /// The response resource.
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.done".
    pub r#type: String,
    /// The response resource.
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.output_item.added".
    pub r#type: String,
    /// The ID of the response to which the item belongs.
    pub response_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The item that was added.
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseOutputItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.output_item.done".
    pub r#type: String,
    /// The ID of the response to which the item belongs.
    pub response_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The completed item.
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseContentPartAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.content_part.added".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item to which the content part was added.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The content part that was added.
    pub part: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseContentPartDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.content_part.done".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item to which the content part was added.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The content part that is done.
    pub part: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseTextDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.text.delta".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The text delta.
    pub delta: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseTextDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.text.done".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The final text content.
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseAudioTranscriptDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.audio_transcript.delta".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The transcript delta.
    pub delta: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseAudioTranscriptDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.audio_transcript.done".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The final transcript of the audio.
    pub transcript: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseAudioDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.audio.delta".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// Base64-encoded audio data delta.
    pub delta: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseAudioDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.audio.done".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseFunctionCallArgumentsDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.function_call_arguments.delta".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the function call.
    pub call_id: String,
    /// The arguments delta as a JSON string.
    pub delta: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseFunctionCallArgumentsDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.function_call_arguments.done".
    pub r#type: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the function call.
    pub call_id: String,
    /// The final arguments as a JSON string.
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RateLimitsUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "rate_limits.updated".
    pub r#type: String,
    /// List of rate limit information.
    pub rate_limits: Vec<RateLimit>,
}
