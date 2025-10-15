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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<ReasoningSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ReasoningSummary {
    #[serde(rename = "summary_text")]
    Text { text: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ReasoningContent {
    #[serde(rename = "reasoning_text")]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageGenerationCall {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeInterpreterCall {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<CodeInterpreterOutput>>,
    pub status: CodeInterpreterStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum CodeInterpreterOutput {
    #[serde(rename = "logs")]
    Logs { logs: String },
    #[serde(rename = "image")]
    Image { url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CodeInterpreterStatus {
    InProgress,
    Completed,
    Incomplete,
    Interpreting,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LocalShellCall {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LocalShellAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum LocalShellAction {
    #[serde(rename = "exec")]
    Exec {
        command: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        env: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_ms: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        working_directory: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct McpToolCall {
    pub id: String,
    pub name: String,
    pub server_label: String,
    pub arguments: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub status: McpStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum McpStatus {
    InProgress,
    Completed,
    Incomplete,
    Calling,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct McpListTools {
    pub id: String,
    pub server_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<McpTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct McpTool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct McpApprovalRequest {
    pub id: String,
    pub name: String,
    pub server_label: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomToolCall {
    pub id: String,
    pub name: String,
    pub call_id: String,
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ComputerToolCall {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ComputerAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_safety_checks: Option<Vec<String>>,
    pub status: InputItemStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ComputerAction {
    #[serde(rename = "click")]
    Click { x: i32, y: i32, button: MouseButton },
    #[serde(rename = "double_click")]
    DoubleClick { x: i32, y: i32 },
    #[serde(rename = "drag")]
    Drag { path: Vec<Coordinate> },
    #[serde(rename = "keypress")]
    KeyPress { keys: Vec<String> },
    #[serde(rename = "move")]
    Move { x: i32, y: i32 },
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "scroll")]
    Scroll {
        x: i32,
        y: i32,
        scroll_x: i32,
        scroll_y: i32,
    },
    #[serde(rename = "type")]
    Type { text: String },
    #[serde(rename = "wait")]
    Wait,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Wheel,
    Back,
    Forward,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LogProbs {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogProb>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ReasoningSummaryPart {
    #[serde(rename = "summary_text")]
    SummaryText { text: String },
}
