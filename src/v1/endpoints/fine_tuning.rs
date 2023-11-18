use serde_json::Value;

use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::fine_tuning::CreateFineTuningJobParameters;
use crate::v1::resources::fine_tuning::FineTuningJob;
use crate::v1::resources::fine_tuning::ListFineTuningJobsParameters;
use crate::v1::resources::fine_tuning::ListFineTuningJobsResponse;

pub struct FineTuning<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Manage fine-tuning jobs to tailor a model to your specific training data.
    pub fn fine_tuning(&self) -> FineTuning {
        FineTuning { client: self }
    }
}

impl FineTuning<'_> {
    /// List your organization's fine-tuning jobs.
    pub async fn list(
        &self,
        query: Option<ListFineTuningJobsParameters>,
    ) -> Result<ListFineTuningJobsResponse, APIError> {
        let response = self
            .client
            .get_with_query("/fine_tuning/jobs", &query)
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let list_fine_tuning_jobs_response: ListFineTuningJobsResponse =
            serde_json::from_value(value.clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_fine_tuning_jobs_response)
    }

    /// Creates a job that fine-tunes a specified model from a given dataset.
    pub async fn create(
        &self,
        parameters: CreateFineTuningJobParameters,
    ) -> Result<FineTuningJob, APIError> {
        let response = self.client.post("/fine_tuning/jobs", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let fine_tuning_job_response: FineTuningJob = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tuning_job_response)
    }
}
