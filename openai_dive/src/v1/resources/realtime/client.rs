use super::{
    default_type_value,
    resources::{item::Item, response::Response, session::Session},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "SessionUpdateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct SessionUpdate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"session.update\")")]
    pub r#type: String,
    /// Session configuration to update.
    pub session: Session,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "InputAudioBufferAppendBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct InputAudioBufferAppend {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"input_audio_buffer.append\")")]
    pub r#type: String,
    /// Base64-encoded audio bytes.
    pub audio: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "InputAudioBufferCommitBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct InputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"input_audio_buffer.commit\")")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "InputAudioBufferClearBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct InputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"input_audio_buffer.clear\")")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ConversationItemCreateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"conversation.item.create\")")]
    pub r#type: String,
    /// The ID of the preceding item after which the new item will be inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
    /// The item to add to the conversation.
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ConversationItemTruncateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ConversationItemTruncate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"conversation.item.truncate\")")]
    pub r#type: String,
    /// The ID of the assistant message item to truncate.
    pub item_id: String,
    /// The index of the content part to truncate.
    pub content_index: u32,
    /// Inclusive duration up to which audio is truncated, in milliseconds.
    pub audio_end_ms: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ConversationItemDeleteBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ConversationItemDelete {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"conversation.item.delete\")")]
    pub r#type: String,
    /// The ID of the item to delete.
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ResponseCreateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ResponseCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"response.create\")")]
    pub r#type: String,
    /// Configuration for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ResponseCancelBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ResponseCancel {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[builder(default = "default_type_value(\"response.cancel\")")]
    pub r#type: String,
}
