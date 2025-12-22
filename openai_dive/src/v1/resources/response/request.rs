use crate::v1::resources::response::items::ComputerToolCallOutput;
use crate::v1::resources::shared::WebSearchContextSize;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    items::{
        ComputerToolCall, FileSearchToolCall, FunctionToolCall, FunctionToolCallOutput, Message,
        Reasoning, WebSearchToolCall,
    },
    response::{ResponseReasoning, ResponseText, Role},
    shared::{ResponseTool, ResponseToolChoice, TruncationStrategy, WebSearchUserLocation},
};

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ResponseParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ResponseParameters {
    /// Text, image, or file inputs to the model, used to generate a response.
    pub input: ResponseInput,
    /// Model ID used to generate the response.
    pub model: String,
    /// Whether to run the model response in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// Reference to a prompt template and its variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
    /// Specify additional output data to include in the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseInclude>>,
    /// serts a system (or developer) message as the first item in the model's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// An upper bound for the number of tokens that can be generated for a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// The unique ID of the previous response to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Configuration options for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ResponseReasoning>,
    /// Whether to store the generated model response for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Whether to stream the response back to the client as it is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Configuration options for a text response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseText>,
    /// How the model should select which tool (or tools) to use when generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ResponseToolChoice>,
    /// An array of tools the model may call while generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ResponseTool>>,
    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// The truncation strategy to use for the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationStrategy>,
    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputMessage {
    pub role: Role,
    pub content: ContentInput,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Prompt {
    /// The unique identifier of the prompt template to use.
    pub id: String,
    /// Optional version of the prompt template.
    pub version: Option<String>,
    /// Optional map of values to substitute in for variables in your prompt.
    pub variables: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ContentInput {
    Text(String),
    List(Vec<ContentItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "input_text")]
    Text { text: String },
    #[serde(rename = "input_image")]
    Image {
        detail: ImageDetailLevel,
        file_id: Option<String>,
        image_url: Option<String>,
    },
    #[serde(rename = "input_file")]
    File {
        file_data: Option<String>,
        file_id: Option<String>,
        filename: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResponseInput {
    Text(String),
    List(Vec<ResponseInputItem>),
}

impl Default for ResponseInput {
    fn default() -> Self {
        ResponseInput::Text(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message(InputMessage),
    ItemReference { id: String },
    Item(InputItem),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum InputItem {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "file_search_call")]
    FileSearchToolCall(FileSearchToolCall),
    #[serde(rename = "computer_call")]
    ComputerToolCall(ComputerToolCall),
    #[serde(rename = "computer_call_output")]
    ComputerToolCallOutput(ComputerToolCallOutput),
    #[serde(rename = "web_search_call")]
    WebSearchToolCall(WebSearchToolCall),
    #[serde(rename = "function_call")]
    FunctionToolCall(FunctionToolCall),
    #[serde(rename = "function_call_output")]
    FunctionToolCallOutput(FunctionToolCallOutput),
    #[serde(rename = "reasoning")]
    Reasoning(Reasoning),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebSearchOption {
    pub search_context_size: Option<WebSearchContextSize>,
    pub user_location: Option<WebSearchUserLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ResponseInclude {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageUrls,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrls,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetailLevel {
    High,
    Low,
    Auto,
}
