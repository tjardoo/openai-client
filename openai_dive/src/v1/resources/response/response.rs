use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::v1::resources::{chat::ReasoningEffort, shared::Usage};

use super::shared::{ResponseFormat, ResponseTool, ResponseToolChoice, TruncationStrategy};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseObject {
    /// Unix timestamp (in seconds) of when this Response was created.
    pub created_at: u32,
    /// An error object returned when the model fails to generate a Response.
    pub error: Option<ResponseError>,
    /// Unique identifier for this Response.
    pub id: String,
    /// Details about why the response is incomplete.
    pub incomplete_details: Option<IncompleteDetails>,
    /// Inserts a system (or developer) message as the first item in the model's context.
    pub instruction: Option<String>,
    /// An upper bound for the number of tokens that can be generated for a response, including visible output tokens and reasoning tokens.
    pub max_output_tokens: Option<u32>,
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: Option<HashMap<String, String>>,
    /// Model ID used to generate the response.
    pub model: String,
    /// The object type of this resource - always set to response.
    pub object: String,
    /// An array of content items generated by the model.
    pub output: Vec<ResponseOutput>,
    /// Whether to allow the model to run tool calls in parallel.
    pub parallel_tool_calls: Option<bool>,
    /// The unique ID of the previous response to the model.
    pub previous_response_id: Option<String>,
    /// Configuration options for reasoning models.
    pub reasoning: Option<ResponseReasoning>,
    /// The status of the response generation.
    pub status: ReasoningStatus,
    /// What sampling temperature to use, between 0 and 2.
    pub temperature: Option<f32>,
    /// Configuration options for a text response from the model.
    pub text: Option<ResponseText>,
    /// How the model should select which tool (or tools) to use when generating a response.
    pub tool_choice: Option<ResponseToolChoice>,
    /// An array of tools the model may call while generating a response.
    pub tools: Vec<ResponseTool>,
    /// An alternative to sampling with temperature.
    pub top_p: Option<f32>,
    /// The truncation strategy to use for the model response.
    pub truncation: Option<TruncationStrategy>,
    /// Represents token usage details.
    pub usage: Usage,
    /// A unique identifier representing your end-user.
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseError {
    /// The error code for the response.
    pub code: String,
    /// A human-readable description of the error.
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IncompleteDetails {
    /// The reason why the response is incomplete.
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutput {
    Message(OutputMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OutputMessage {
    pub id: String,
    pub role: Role,
    pub status: MessageStatus,
    pub content: Vec<OutputContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseReasoning {
    /// Constrains effort on reasoning for reasoning models.
    pub effort: Option<ReasoningEffort>,
    /// A summary of the reasoning performed by the model.
    pub generate_summary: Option<ReasoningSummary>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseText {
    /// An object specifying the format that the model must output.
    pub format: ResponseFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    Concise,
    Detailed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningStatus {
    Completed,
    Failed,
    InProgress,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
    Developer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum OutputContent {
    #[serde(rename = "output_text")]
    Text {
        text: String,
        // annotations: Vec<Annotation>,
    },
    #[serde(rename = "refusal")]
    Refusal { refusal: String },
}
