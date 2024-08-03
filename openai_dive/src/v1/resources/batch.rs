use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Batch {
    /// The unique identifier for the batch.
    pub id: String,
    /// The object type, which is always "batch".
    pub object: String,
    /// The OpenAI API endpoint used by the batch.
    pub endpoint: String,
    /// For batch jobs that have failed, this will contain more information on the cause of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchErrors>,
    /// The ID of the input file for the batch.
    pub input_file_id: String,
    /// The time frame within which the batch should be processed.
    pub completion_window: BatchCompletionWindow,
    /// The current status of the batch.
    pub status: BatchStatus,
    /// The ID of the file containing the outputs of successfully executed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_id: Option<String>,
    /// The ID of the file containing the outputs of requests with errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file_id: Option<String>,
    /// The Unix timestamp (in seconds) for when the batch was created.
    pub created_at: u32,
    /// The Unix timestamp (in seconds) for when the batch started processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizing_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelling_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<u32>,
    /// The request counts for different statuses within the batch.
    pub request_counts: BatchRequestCounts,
    /// Optional custom metadata of the batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateBatchParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateBatchParameters {
    /// The ID of an uploaded file that contains requests for the new batch.
    pub input_file_id: String,
    /// The endpoint to be used for all requests in the batch.
    /// Currently '/v1/chat/completions', '/v1/embeddings', and '/v1/completions' are supported.
    pub endpoint: String,
    /// The time frame within which the batch should be processed. Currently only "24h" is supported.
    pub completion_window: BatchCompletionWindow,
    /// Optional custom metadata for the batch.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BatchErrors {
    /// The object type, which is always "list".
    pub object: String,
    pub data: Vec<BatchError>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BatchError {
    /// An error code identifying the error type.
    pub code: String,
    /// A human-readable message providing more details about the error.
    pub message: String,
    /// The name of the parameter that caused the error, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// The line number of the input file where the error occurred, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch.
    pub total: u32,
    /// Number of requests that have been completed successfully.
    pub completed: u32,
    /// Number of requests that have failed.
    pub failed: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    Validating,
    Failed,
    InProgress,
    Finalizing,
    Completed,
    Expired,
    Cancelling,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub enum BatchCompletionWindow {
    #[default]
    #[serde(rename = "24h")]
    H24,
}
