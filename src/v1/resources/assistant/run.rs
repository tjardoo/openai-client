use crate::v1::resources::chat::ToolCall;
use crate::v1::resources::{assistant::assistant::AssistantTools, shared::Usage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// The model that the assistant used for this run.
    pub model: String,
    /// The instructions that the assistant used for this run.
    pub instructions: String,
    /// The list of tools that the assistant used for this run.
    pub tools: Vec<AssistantTools>,
    /// The list of File IDs the assistant used for this run.
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Usage statistics related to the run.
    /// This value will be null if the run is not in a terminal state (i.e. in_progress, queued, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
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
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
