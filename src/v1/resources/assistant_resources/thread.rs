use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Thread {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'thread'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: u32,
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateThreadParameters {
    /// A list of messages to start the thread with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ThreadMessage>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModifyThreadParameters {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadMessage {
    /// The role of the entity that is creating the message. Currently only user is supported.
    pub role: ThreadMessageRole,
    /// The content of the message.
    pub content: String,
    /// A list of File IDs that the message should use.
    /// There can be a maximum of 10 files attached to a message.
    /// Useful for tools like retrieval and code_interpreter that can access and use files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ThreadMessageRole {
    User,
}
