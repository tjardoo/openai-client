use derive_builder::Builder;
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
    /// The status of the message, which can be either in_progress, incomplete, or completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageStatus>,
    /// On an incomplete message, details about why the message is incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteMessage>,
    /// The Unix timestamp (in seconds) for when the message was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the message was marked as incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_at: Option<u32>,
    /// The entity that produced the message. One of user or assistant.
    pub role: MessageRole,
    /// The content of the message in array of text and/or images.
    pub content: Vec<MessageContent>,
    /// If applicable, the ID of the assistant that authored this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// The ID of the run associated with the creation of this message.
    /// Value is null when messages are created manually using the create message or create thread endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// A list of files attached to the message, and the tools they were added to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateMessageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateMessageParameters {
    /// The role of the entity that is creating the message.
    pub role: MessageRole,
    /// The content of the message.
    pub content: String,
    /// A list of files attached to the message, and the tools they should be added to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ModifyMessageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModifyMessageParameters {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "MessageAttachmentBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct MessageAttachment {
    /// The ID of the file to attach to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The tools to add this file to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<MessageTool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum MessageTool {
    CodeInterpreter {
        /// The type of tool being defined: 'code_interpreter'.
        r#type: String,
    },
    FileSearch {
        /// The type of tool being defined: 'file_search'.
        r#type: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageFile {
    /// The File ID of the image in the message content.
    /// Set purpose="vision" when uploading the File if you need to later display the file content.
    pub file_id: String,
    /// Specifies the detail level of the image if specified by the user.
    /// 'low' uses fewer tokens, you can opt in to high resolution using 'high'.
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUrl {
    /// The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp.
    pub url: String,
    /// Specifies the detail level of the image if specified by the user.
    /// 'low' uses fewer tokens, you can opt in to high resolution using 'high'.
    pub detail: String,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FilePath {
    /// The ID of the file that was generated.
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageFile {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'thread.message.file'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the message file was created.
    pub created_at: u32,
    /// The ID of the message that the File is attached to.
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IncompleteMessage {
    /// The reason the message is incomplete.
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    #[default]
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    InProgress,
    Incomplete,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    ImageFile {
        /// Always 'image_file'.
        r#type: String,
        /// Object containing the image file's ID.
        image_file: ImageFile,
    },
    ImageUrl {
        /// The type of the content part.
        r#type: String,
        /// Object containing the image file's URL.
        image_url: ImageUrl,
    },
    Text {
        /// Always 'text'.
        r#type: String,
        /// Object containing the text.
        text: Text,
    },
    Refusal {
        /// Always 'refusal'.
        r#type: String,
        /// The refusal text.
        refusal: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextAnnotation {
    FileCitation(FileCitationAnnotation),
    FilePath(FilePathAnnotation),
}
