use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::v1::resources::shared::{LastError, Usage};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStep {
    /// The identifier of the run step, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always thread.run.step.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run step was created.
    pub created_at: u32,
    /// The ID of the assistant associated with the run step.
    pub assistant_id: String,
    /// The ID of the thread that was run.
    pub thread_id: String,
    /// The type of run step, which can be either message_creation or tool_calls.
    pub r#type: RunStepType,
    /// The status of the run step, which can be either in_progress, cancelled, failed, completed, or expired.
    pub status: RunStepStatus,
    /// The details of the run step.
    pub step_details: RunStepDetails,
    /// The last error associated with this run step. Will be null if there are no errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    /// The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run step failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the run step completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u32>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Usage statistics related to the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RunStepDetails {
    MessageCreation(MessageCreationDetails),
    ToolCalls(ToolCallsDetails),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageCreationDetails {
    /// Always message_creation.
    pub r#type: String,
    pub message_creation: MessageCreation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageCreation {
    /// The ID of the message that was created by this run step.
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolCallsDetails {
    /// Always tool_calls.
    pub r#type: String,
    pub tool_calls: Vec<RunStepToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStepType {
    MessageCreation,
    ToolCalls,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStepStatus {
    InProgress,
    Cancelled,
    Failed,
    Completed,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RunStepToolCall {
    CodeInterpreter(RunStepCodeInterpreterTool),
    FileSearch(RunStepFileSearchTool),
    Function(RunStepFunctionTool),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStepCodeInterpreterTool {
    /// The ID of the tool call.
    pub id: String,
    /// The type of tool call. This is always going to be code_interpreter for this type of tool call.
    pub r#type: String,
    /// The Code Interpreter tool call definition.
    pub code_interpreter: RunStepCodeInterpreterToolObject,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStepCodeInterpreterToolObject {
    /// The input to the Code Interpreter tool call.
    pub input: String,
    /// The outputs from the Code Interpreter tool call.
    /// Code Interpreter can output one or more items, including text (logs) or images (image).
    /// Each of these are represented by a different object type.
    pub outputs: Vec<RunStepCodeInterpreterToolOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RunStepCodeInterpreterToolOutput {
    CodeInterpreterLogOutput {
        /// Always logs.
        r#type: String,
        /// The text output from the Code Interpreter tool call.
        logs: String,
    },
    CodeInterpreterImageOutput {
        /// Always image.
        r#type: String,
        image: CodeInterpreterImageOutputImage,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeInterpreterImageOutputImage {
    /// The file ID of the image.
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStepFileSearchTool {
    /// The ID of the tool call object.
    pub id: String,
    /// The type of tool call. This is always going to be file_search for this type of tool call.
    pub r#type: String,
    /// For now, this is always going to be an empty object.
    pub file_search: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStepFunctionTool {
    /// The ID of the tool call object.
    pub id: String,
    /// The type of tool call. This is always going to be function for this type of tool call.
    pub r#type: String,
    /// The definition of the function that was called.
    pub function: RunStepFunctionToolFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RunStepFunctionToolFunction {
    /// The name of the function.
    pub name: String,
    /// The arguments passed to the function.
    pub arguments: String,
    /// The output of the function. This will be null if the outputs have not been submitted yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}
