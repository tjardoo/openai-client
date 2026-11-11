use crate::v1::resources::response::items::{ComputerToolCallOutput, InputItemStatus};
use crate::v1::resources::response::shared::Annotation;
use crate::v1::resources::shared::WebSearchContextSize;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    items::{
        ComputerToolCall, FileSearchToolCall, FunctionToolCall, FunctionToolCallOutput, Reasoning,
        WebSearchToolCall,
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
    /// Configures prompt caching for `gpt-5.6` and later models.
    ///
    /// Use this to choose automatic or explicit cache boundaries, set the default cache lifetime, or fill the cache
    /// without generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<PromptCacheOptions>,
    /// Specify additional output data to include in the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseInclude>>,
    /// Inserts a system (or developer) message as the first item in the model's context.
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
    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// The truncation strategy to use for the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationStrategy>,
    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The service tier used for processing the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InputItemStatus>,
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
    Text {
        text: String,
        /// Marks the end of this content part as a cacheable prompt prefix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
    },
    #[serde(rename = "input_image")]
    Image {
        detail: ImageDetailLevel,
        file_id: Option<String>,
        image_url: Option<String>,
        /// Marks the end of this content part as a cacheable prompt prefix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
    },
    #[serde(rename = "input_file")]
    File {
        file_data: Option<String>,
        file_id: Option<String>,
        filename: Option<String>,
        /// Marks the end of this content part as a cacheable prompt prefix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
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

/// Controls how the Responses API caches reusable prompt prefixes.
///
/// Caching applies only when the prompt contains at least 1,024 input tokens. Cache hits reduce the cost of repeated
/// input, while cache writes cost more than normal input.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PromptCacheOptions {
    /// Chooses where the API may cache the prompt. The default, [`PromptCacheMode::Implicit`], adds an automatic
    /// boundary in addition to any explicit breakpoints. [`PromptCacheMode::Explicit`] uses only explicit breakpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PromptCacheMode>,
    /// Sets the cache lifetime for the automatic boundary and for explicit breakpoints without their own `ttl`.
    /// Defaults to 30 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PromptCacheTtl>,
    /// Set to `true` to fill the prompt cache without generating model output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prewarm: Option<bool>,
}

/// Determines whether the API adds an automatic prompt cache boundary.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PromptCacheMode {
    Implicit,
    Explicit,
}

/// How long a cached prompt prefix remains available.
///
/// The 30-minute lifetime is the default and its writes cost 1.25 times normal input. Twelve-hour writes cost 2.5
/// times normal input. Cache hits receive the cached-input discount for either lifetime.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum PromptCacheTtl {
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "12h")]
    TwelveHours,
}

/// Marks the end of an input content part as a cacheable prompt prefix.
///
/// Include the same breakpoint in every request that should reuse the prefix; the API checks only breakpoints present
/// in the current request. The prompt must contain at least 1,024 input tokens for caching to apply.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PromptCacheBreakpoint {
    pub mode: PromptCacheBreakpointMode,
    /// Overrides [`PromptCacheOptions::ttl`] for this boundary. When omitted, the request-wide lifetime applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PromptCacheTtl>,
}

/// The kind of explicit prompt cache boundary.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PromptCacheBreakpointMode {
    #[default]
    Explicit,
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
    ItemReference {
        id: String,
    },
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
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetailLevel {
    High,
    Low,
    Auto,
}
