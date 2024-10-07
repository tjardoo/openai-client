use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
    pub call_id: Option<String>,
    /// The name of the function being called (for "function_call" items).
    pub name: Option<String>,
    /// The arguments of the function call (for "function_call" items).
    pub arguments: Option<String>,
    /// The output of the function call (for "function_call_output" items).
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
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
