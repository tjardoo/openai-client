use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::message::MessageAttachment;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Thread {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'thread'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: u32,
    /// A set of resources that are made available to the assistant's tools in this thread.
    /// The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs,
    /// while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ThreadToolResource>,
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateThreadParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateThreadParameters {
    /// A list of messages to start the thread with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ThreadMessage>>,
    /// A set of resources that are made available to the assistant's tools in this thread.
    /// The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs,
    /// while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ThreadToolResource>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Builder, PartialEq)]
#[builder(name = "ModifyThreadParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModifyThreadParameters {
    /// A set of resources that are made available to the assistant's tools in this thread.
    /// The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs,
    /// while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ThreadToolResource>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ThreadMessageBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ThreadMessage {
    /// The role of the entity that is creating the message. Currently only user is supported.
    pub role: ThreadMessageRole,
    /// The content of the message.
    pub content: String,
    /// A list of files attached to the message, and the tools they should be added to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ThreadToolResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_interpreter: Option<CodeInterpreter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<FileSearch>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeInterpreter {
    /// A list of file IDs made available to the code_interpreter tool. There can be a maximum of 20 files associated with the tool.
    pub file_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSearch {
    /// The vector store attached to this thread. There can be a maximum of 1 vector store attached to the thread.
    pub vector_store_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ThreadMessageRole {
    #[default]
    User,
    Assistant,
}
