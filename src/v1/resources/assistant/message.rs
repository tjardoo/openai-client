use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'thread.message'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the message was created.
    pub created_at: u32,
    /// The thread ID that this message belongs to.
    pub thread_id: String,
    /// The entity that produced the message. One of 'user' or 'assistant'.
    pub role: MessageRole,
    /// The content of the message in array of text and/or images.
    pub content: Vec<MessageContent>,
    /// If applicable, the ID of the assistant that authored this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// If applicable, the ID of the run associated with the authoring of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// A list of file IDs that the assistant should use. Useful for tools like 'retrieval' and 'code_interpreter'
    /// that can access files. A maximum of 10 files can be attached to a message.
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageFileContent {
    /// Always 'image_file'.
    pub r#type: String,
    /// Object containing the image file's ID.
    pub image_file: ImageFile,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageFile {
    /// The File ID of the image in the message content.
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextContent {
    /// Always 'text'.
    pub r#type: String,
    /// Object containing the text.
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Text {
    /// The data that makes up the text.
    pub value: String,
    /// Object containing the text.
    pub annotations: Vec<TextAnnotation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileCitationAnnotation {
    /// Always 'file_citation'.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: String,
    /// The file citation object.
    pub file_citation: FileCitation,
    /// Start index of the text in the message content that needs to be replaced.
    pub start_index: u32,
    /// End index of the text in the message content that needs to be replaced.
    pub end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FilePathAnnotation {
    /// Always 'file_path'.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: String,
    /// The file path object.
    pub file_path: FilePath,
    /// Start index of the text in the message content that needs to be replaced.
    pub start_index: u32,
    /// End index of the text in the message content that needs to be replaced.
    pub end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileCitation {
    /// The ID of the specific File the citation is from.
    pub file_id: String,
    /// The specific quote in the file.
    pub quote: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FilePath {
    /// The ID of the file that was generated.
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateMessageParameters {
    /// The role of the entity that is creating the message. Currently 'only' user is supported.
    pub role: MessageRole,
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
pub struct ModifyMessageParameters {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListMessagesResponse {
    /// The object type, which is always 'list'.
    pub object: String,
    /// The list of assistant files.
    pub data: Vec<Message>,
    /// ID of the first object in the list.
    pub first_id: String,
    /// ID of the last object in the list.
    pub last_id: String,
    /// Indicates whether there are more assistant files to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    ImageFile(ImageFileContent),
    Text(TextContent),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextAnnotation {
    FileCitation(FileCitationAnnotation),
    FilePath(FilePathAnnotation),
}
