use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{request::ImageDetailLevel, response::Role, shared::Annotation};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub role: Role,
    pub content: MessageContent,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSearchToolCall {
    pub id: String,
    pub queries: Vec<String>,
    pub name: Option<String>,
    pub results: Option<Vec<FileSearchResult>>,
    pub status: FileSearchStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSearchResult {
    pub file_id: String,
    pub filename: String,
    pub score: Option<f32>,
    pub text: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebSearchToolCall {
    pub id: String,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FunctionToolCall {
    pub id: String,
    pub call_id: String,
    pub name: String,
    pub arguments: String,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FunctionToolCallOutput {
    pub id: Option<String>,
    pub call_id: String,
    pub output: String,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Reasoning {
    pub id: String,
    pub summary: Vec<ReasoningSummary>,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ReasoningSummary {
    #[serde(rename = "summary_text")]
    Text { text: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputItemStatus {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileSearchStatus {
    InProgress,
    Searching,
    Completed,
    Incomplete,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "input_text")]
    InputText {
        text: String,
    },
    #[serde(rename = "input_image")]
    Image {
        file_id: Option<String>,
        image_url: Option<String>,
        detail: ImageDetailLevel,
    },
    #[serde(rename = "input_file")]
    File {
        file_id: Option<String>,
        filename: Option<String>,
        file_data: Option<String>,
    },
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        annotations: Vec<Annotation>,
    },
    Refusal {
        refusal: String,
    },
}
