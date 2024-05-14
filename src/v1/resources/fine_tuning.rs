use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: u32,
    /// For fine-tuning jobs that have failed, this will contain more information on the cause of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FineTuningJobError>,
    /// The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<u32>,
    /// The hyperparameters used for the fine-tuning job.
    pub hyperparameters: FineTuningJobHyperparameters,
    /// The base model that is being fine-tuned.
    pub model: String,
    /// The object type, which is always "fine_tuning.job".
    pub object: String,
    /// The organization that owns the fine-tuning job.
    pub organization_id: String,
    /// The compiled results file ID(s) for the fine-tuning job.
    pub result_files: Vec<String>,
    /// The current status of the fine-tuning job, which can be either validating_files, queued, running, succeeded, failed, or cancelled.
    pub status: FineTuningJobStatus,
    /// The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trained_tokens: Option<u32>,
    /// The file ID used for training.
    pub training_file: String,
    /// The file ID used for validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    /// A list of integrations to enable for this fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<FineTuningIntegration>>,
    /// The seed used for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    /// The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish.
    /// The value will be null if the fine-tuning job is not running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_finish: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateFineTuningJobParameters {
    /// The name of the model to fine-tune.
    pub model: String,
    /// The ID of an uploaded file that contains training data.
    pub training_file: String,
    /// The hyperparameters used for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<FineTuningJobHyperparameters>,
    /// A string of up to 18 characters that will be added to your fine-tuned model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// The ID of an uploaded file that contains validation data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    /// A list of integrations to enable for your fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<FineTuningIntegration>,
    /// The seed controls the reproducibility of the job.
    /// Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.
    /// If a seed is not specified, one will be generated for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJobEvent {
    /// The job event identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) when the fine tuning job event was created.
    pub created_at: u32,
    /// The fine-tuning job event level, which can be either info or error.
    pub level: String,
    /// The fine-tuning job event message.
    pub message: String,
    /// The object type, which is always "fine_tuning.job.event".
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJobCheckpoint {
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub created_at: u32,
    /// The name of the fine-tuned checkpoint model that is created.
    pub fine_tuned_model_checkpoint: String,
    /// The step number that the checkpoint was created at.
    pub step_number: u32,
    /// Metrics at the step number during the fine-tuning job.
    pub metrics: FineTuningJobCheckpointMetrics,
    /// The name of the fine-tuning job that this checkpoint was created from.
    pub fine_tuning_job_id: String,
    /// The object type, which is always "fine_tuning.job.checkpoint".
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJobCheckpointMetrics {
    pub step: u32,
    pub train_loss: f32,
    pub train_mean_token_accuracy: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_loss: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_mean_token_accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_valid_loss: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_valid_mean_token_accuracy: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFineTuningJobsResponse {
    /// The object type, which is always "fine_tuning.job.event".
    pub object: String,
    /// The list of fine-tuning jobs.
    pub data: Vec<FineTuningJob>,
    /// Indicates whether there are more fine-tuning jobs to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFineTuningJobEventsResponse {
    /// The object type, which is always "fine_tuning.job.event".
    pub object: String,
    /// The list of fine-tuning job events.
    pub data: Vec<FineTuningJobEvent>,
    /// Indicates whether there are more fine-tuning job events to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFineTuningCheckpointsResponse {
    /// The object type, which is always "list".
    pub object: String,
    /// The list of fine-tuning checkpoints.
    pub data: Vec<FineTuningJobCheckpoint>,
    /// The ID of the first checkpoint in the list.
    pub first_id: Option<String>,
    /// The ID of the last checkpoint in the list.
    pub last_id: Option<String>,
    /// Indicates whether there are more fine-tuning checkpoints to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJobError {
    /// The error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The parameter that caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningIntegration {
    /// The type of the integration being enabled for the fine-tuning job.
    pub r#type: String,
    /// The settings for your integration with Weights and Biases.
    /// This payload specifies the project that metrics will be sent to.
    pub wandb: WandB,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WandB {
    /// The name of the project that the new run will be created under.
    pub project: String,
    /// A display name to set for the run. If not set, we will use the Job ID as the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The entity to use for the run.
    /// This allows you to set the team or username of the WandB user that you would like associated with the run.
    /// If not set, the default entity for the registered WandB API key is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// A list of tags to be attached to the newly created run. These tags are passed through directly to WandB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FineTuningJobHyperparameters {
    /// Number of examples in each batch.
    /// A larger batch size means that model parameters are updated less frequently, but with lower variance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<LearningRateMultiplier>,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: NEpochs,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BatchSize {
    #[serde(rename = "auto")]
    Auto,
    Integer(u32),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum LearningRateMultiplier {
    #[serde(rename = "auto")]
    Auto,
    Integer(u32),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NEpochs {
    #[serde(rename = "auto")]
    Auto,
    Integer(u32),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningJobStatus {
    ValidatingFiles,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
