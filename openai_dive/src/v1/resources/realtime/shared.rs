use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub(crate) fn default_type_value(value: &'static str) -> String {
    value.to_string()
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ConversationBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct Conversation {
    // @todo
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "SessionBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct Session {
    pub id: String,
    #[builder(default = "default_type_value(\"realtime.session\")")]
    pub r#type: String,
    /// The default model used for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The set of modalities the model can respond with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,
    /// The default system instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// The voice the model uses to respond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    /// The format of input audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<AudioFormat>,
    /// The format of output  audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<AudioFormat>,
    /// Configuration for input audio transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<InputAudioTranscription>,
    /// Configuration for turn detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<TurnDetection>,
    /// Tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// How the model chooses tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    /// Sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of output tokens (integer or "inf").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorDetail {
    /// The type of error (e.g., "invalid_request_error", "server_error").
    pub error_type: String,
    /// Error code, if any.
    pub code: Option<String>,
    /// A human-readable error message.
    pub message: String,
    /// Parameter related to the error, if any.
    pub param: Option<String>,
    /// The event_id of the client event that caused the error, if applicable.
    pub event_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TranscriptionError {
    /// The type of error.
    pub r#type: String,
    /// Error code, if any.
    pub code: Option<String>,
    /// A human-readable error message.
    pub message: String,
    /// Parameter related to the error, if any.
    pub param: Option<String>,
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
pub struct Output {
    pub id: String,
    pub object: String,
    pub r#type: String,
    pub status: String,
    pub role: String,
    pub content: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
pub struct InputAudioTranscription {
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

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ItemBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct Item {
    /// The unique ID of the item.
    pub id: String,
    /// The type of the item ("message", "function_call", "function_call_output").
    pub r#type: ItemType,
    /// The status of the item ("completed", "in_progress", "incomplete").
    pub status: ItemStatus,
    /// The role of the message sender ("user", "assistant", "system").
    pub role: ItemRole,
    /// The content of the message.
    pub content: Vec<Content>,
    /// The ID of the function call (for "function_call" items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// The name of the function being called (for "function_call" items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The arguments of the function call (for "function_call" items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    /// The output of the function call (for "function_call_output" items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Content {
    /// The content type ("input_text", "input_audio", "text", "audio").
    pub r#type: ContentType,
    /// The text content.
    pub text: Option<String>,
    /// Base64-encoded audio bytes.
    pub audio: Option<String>,
    /// The transcript of the audio.
    pub transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ResponseBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct Response {
    /// The modalities for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,
    /// Instructions for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// The voice the model uses to respond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    /// The format of output audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<String>,
    /// Tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// How the model chooses tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    /// Sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of output tokens (integer or "inf").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MaxTokens {
    #[serde(rename = "inf")]
    Inf,
    Integer(u32),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    #[default]
    Text,
    Audio,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    #[default]
    InputText,
    InputAudio,
    Text,
    Audio,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    #[default]
    Message,
    FunctionCall,
    FunctionCallOutput,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    #[default]
    Completed,
    InProgress,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemRole {
    #[default]
    User,
    Assistant,
    System,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    Alloy,
    Echo,
    Shimmer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    #[serde(rename = "pcm16")]
    PCM16,
    #[serde(rename = "g711-ulaw")]
    G711ULAW,
    #[serde(rename = "g711-alaw")]
    G711ALAW,
}
