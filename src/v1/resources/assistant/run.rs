use crate::v1::resources::{assistant::assistant::AssistantTools, shared::Usage};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::thread::CreateThreadParameters;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Run {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'thread.run'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run was created.
    pub created_at: u32,
    /// The ID of the thread that was executed on as a part of this run.
    pub thread_id: String,
    /// The ID of the assistant used for execution of this run.
    pub assistant_id: String,
    /// The status of the run, which can be either queued, in_progress, requires_action,
    /// cancelling, cancelled, failed, completed, or expired.
    pub status: RunStatus,
    /// Details on the action required to continue the run. Will be null if no action is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_action: Option<RunAction>,
    /// The last error associated with this run. Will be null if there are no errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<RunError>,
    /// The Unix timestamp (in seconds) for when the run will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run was started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u32>,
    /// Details on why the run is incomplete. Will be null if the run is not incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    /// The model that the assistant used for this run.
    pub model: String,
    /// The instructions that the assistant used for this run.
    pub instructions: String,
    /// The list of tools that the assistant used for this run.
    pub tools: Vec<AssistantTools>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Usage statistics related to the run.
    /// This value will be null if the run is not in a terminal state (i.e. in_progress, queued, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// The sampling temperature used for this run. If not set, defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// The nucleus sampling value used for this run. If not set, defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// The maximum number of prompt tokens specified to have been used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_prompt_tokens: Option<u32>,
    /// The maximum number of completion tokens specified to have been used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run.
    pub truncation_strategy: TruncationStrategy,
    /// Controls which (if any) tool is called by the model. none means the model will not call any tools and instead generates a message.
    /// auto is the default value and means the model can pick between generating a message or calling one or more tools.
    /// required means the model must call one or more tools before responding to the user.
    pub tool_choice: ToolChoice,
    /// Whether to enable parallel function calling during tool use.
    pub parallel_tool_calls: bool,
    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<RunResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunAction {
    /// For now, this is always 'submit_tool_outputs'.
    pub r#type: String,
    /// Details on the tool outputs needed for this run to continue.
    pub submit_tool_outputs: RunActionSubmitToolOutput,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunActionSubmitToolOutput {
    /// A list of the relevant tool calls.
    pub tool_calls: Vec<RunToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of tool call the output is required for. For now, this is always function.
    pub r#type: String,
    /// The function definition.
    pub function: RunToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunToolCallFunction {
    /// The name of the function.
    pub name: String,
    /// The arguments that the model expects you to pass to the function.
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateThreadAndRunParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateThreadAndRunParameters {
    #[serde(flatten)]
    pub run: CreateRunParameters,
    pub thread: CreateThreadParameters,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateRunParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateRunParameters {
    /// The ID of the assistant to use to execute this run.
    pub assistant_id: String,
    /// The ID of the Model to be used to execute this run.
    /// If a value is provided here, it will override the model associated with the assistant.
    /// If not, the model associated with the assistant will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Appends additional instructions at the end of the instructions for the run.
    /// This is useful for modifying the behavior on a per-run basis without overriding other instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_instructions: Option<String>,
    /// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTools>>,
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// If true, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a data: [DONE] message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// The maximum number of prompt tokens that may be used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_prompt_tokens: Option<u32>,
    /// The maximum number of completion tokens that may be used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation_strategy: Option<TruncationStrategy>,
    /// Controls which (if any) tool is called by the model. none means the model will not call any tools and instead generates a message.
    /// auto is the default value and means the model can pick between generating a message or calling one or more tools.
    /// required means the model must call one or more tools before responding to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to enable parallel function calling during tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<RunResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ModifyRunParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModifyRunParameters {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListRunsResponse {
    /// The object type, which is always 'list'.
    pub object: String,
    /// The list of runs.
    pub data: Vec<Run>,
    /// ID of the first object in the list.
    pub first_id: String,
    /// ID of the last object in the list.
    pub last_id: String,
    /// Indicates whether there are more runs to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunErrorCode {
    ServerError,
    RateLimitExceeded,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunError {
    /// One of 'server_error' or 'rate_limit_exceeded'.
    pub code: RunErrorCode,
    /// A human-readable description of the error.
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IncompleteDetails {
    /// The reason why the run is incomplete.
    /// This will point to which specific token limit was reached over the course of the run.
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TruncationStrategy {
    /// The truncation strategy to use for the thread. The default is auto.
    /// If set to last_messages, the thread will be truncated to the n most recent messages in the thread.
    /// When set to auto, messages in the middle of the thread will be dropped to fit the context length of the model, max_prompt_tokens.
    pub r#type: String,
    /// The number of most recent messages from the thread when constructing the context for the run.
    pub last_messages: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    #[serde(untagged)]
    JsonObject {
        /// The type of the tool. If type is function, the function name must be set
        #[serde(rename = "type")]
        r#type: String,
        function: ToolChoiceFunction,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunResponseFormat {
    Auto,
    #[serde(untagged)]
    JsonObject {
        /// Must be one of text or json_object.
        #[serde(rename = "type")]
        r#type: String,
    },
}
