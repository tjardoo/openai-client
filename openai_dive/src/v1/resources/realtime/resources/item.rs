use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Item {
    /// The unique ID of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the item ("message", "function_call", "function_call_output").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemType>,
    /// The status of the item ("completed", "in_progress", "incomplete").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// The role of the message sender ("user", "assistant", "system").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ItemRole>,
    /// The content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ItemContent>>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemContent {
    /// The content type ("input_text", "input_audio", "text", "audio").
    pub r#type: ContentType,
    /// The text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Base64-encoded audio bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The transcript of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Message,
    FunctionCall,
    FunctionCallOutput,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemRole {
    User,
    Assistant,
    System,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    InputText,
    InputAudio,
    OutputAudio,
    Text,
    Audio,
}
