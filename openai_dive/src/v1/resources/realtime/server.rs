use serde::{Deserialize, Serialize};

use super::shared::{Content, Item, Usage};

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
pub struct ErrorDetail {
    /// The type of error (e.g., "invalid_request_error", "server_error").
    pub error_type: String,
    /// Error code, if any.
    pub code: String,
    /// A human-readable error message.
    pub message: String,
    /// Parameter related to the error, if any.
    pub param: String,
    /// The event_id of the client event that caused the error, if applicable.
    pub event_id: String,
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
pub struct Session {
    /// The unique ID of the session.
    pub id: String,
    /// The object type, must be "realtime.session".
    pub object: String,
    /// The default model used for this session.
    pub model: String,
    /// The set of modalities the model can respond with.
    pub modalities: Vec<String>,
    /// The default system instructions.
    pub instructions: String,
    /// The voice the model uses to respond.
    pub voice: String,
    /// The format of input audio.
    pub input_audio_format: String,
    /// The format of output audio.
    pub output_audio_format: String,
    /// Configuration for input audio transcription.
    pub input_audio_transcription: Option<InputAudioTranscription>,
    /// Configuration for turn detection.
    pub turn_detection: TurnDetection,
    /// Tools (functions) available to the model.
    pub tools: Vec<Tool>,
    /// How the model chooses tools.
    pub tool_choice: String,
    /// Sampling temperature.
    pub temperature: f32,
    /// Maximum number of output tokens.
    pub max_output_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputAudioTranscription {
    /// Whether input audio transcription is enabled.
    pub enabled: bool,
    /// The model used for transcription.
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TurnDetection {
    /// The type of turn detection ("server_vad" or "none").
    pub r#type: String,
    /// Activation threshold for VAD.
    pub threshold: f32,
    /// Audio included before speech starts (in milliseconds).
    pub prefix_padding_ms: u32,
    /// Duration of silence to detect speech stop (in milliseconds).
    pub silence_duration_ms: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tool {
    /// The type of the tool.
    pub r#type: String,
    /// The name of the function.
    pub name: String,
    /// The description of the function.
    pub description: String,
    /// Parameters of the function in JSON Schema.
    pub parameters: String,
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

pub struct ResponseCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.created".
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
pub struct ResponseDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "response.done".
    pub r#type: String,
    /// The response resource.
    pub response: Response,
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
pub struct RateLimitsUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be "rate_limits.updated".
    pub r#type: String,
    /// List of rate limit information.
    pub rate_limits: Vec<RateLimit>,
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
pub struct RateLimit {
    /// The name of the rate limit ("requests", "tokens", "input_tokens", "output_tokens").
    pub name: RateLimitName,
    /// The maximum allowed value for the rate limit.
    pub limit: u32,
    /// The remaining value before the limit is reached.
    pub remaining: u32,
    /// Seconds until the rate limit resets.
    pub reset_seconds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitName {
    Requests,
    Tokens,
    InputTokens,
    OutputTokens,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    /// The unique ID of the response.
    pub id: String,
    /// The object type, must be "realtime.response".
    pub object: String,
    /// The status of the response ("in_progress").
    pub status: String,
    /// Additional details about the status.
    pub status_details: Option<String>,
    /// The list of output items generated by the response.
    pub output: Vec<Output>,
    /// Usage statistics for the response.
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Output {
    pub id: String,
    pub object: String,
    pub r#type: String,
    pub status: String,
    pub role: String,
    pub content: Vec<Content>,
}
