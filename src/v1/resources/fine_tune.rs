use serde::{Serialize, Deserialize};
use super::file::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFineTuneParameters {
    pub training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    pub model: String,
    // @todo implement missing parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FineTune {
    pub id: String,
    pub model: String,
    pub created_at: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<FineTuneEvent>>,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<File>,
    pub status: String,
    pub validation_files: Vec<File>,
    pub training_files: Vec<File>,
    pub updated_at: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FineTuneEvent {
    pub object: String,
    pub created_at: u32,
    pub level: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hyperparams {
    pub batch_size: Option<u32>,
    pub learning_rate_multiplier: Option<f32>,
    pub n_epochs: u32,
    pub prompt_loss_weight: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedFineTuneModel {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
