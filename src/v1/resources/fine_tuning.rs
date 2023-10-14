use crate::v1::resources::file::File;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateFineTuningParameters {
    pub model: Option<String>,
    pub training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparams: Option<Hyperparams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuningJob {
    pub id: String,
    pub created_at: u32,
    pub error: Option<String>,
    pub fine_tuned_model: Option<String>,
    pub finished_at: Option<u32>,
    pub hyperparams: Hyperparams,
    pub model: String,
    pub organization_id: String,
    pub result_files: Vec<File>,
    pub status: Status,
    pub trained_tokens: Option<u32>,
    pub training_file: String,
    pub validation_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuningJobEvent {
    pub id: String,
    pub created_at: u32,
    pub level: String,
    pub message: String,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hyperparams {
    pub n_epochs: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[serde(rename = "validating_files")]
    ValidatingFiles,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
