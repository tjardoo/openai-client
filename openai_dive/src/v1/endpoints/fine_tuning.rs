use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::fine_tuning::CreateFineTuningJobParameters;
use crate::v1::resources::fine_tuning::FineTuningJob;
use crate::v1::resources::fine_tuning::FineTuningJobCheckpoint;
use crate::v1::resources::fine_tuning::FineTuningJobEvent;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;
use serde_json::Value;

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
    /// Creates a job that fine-tunes a specified model from a given dataset.
    pub async fn create(
        &self,
        parameters: CreateFineTuningJobParameters,
    ) -> Result<FineTuningJob, APIError> {
        let response = self.client.post("/fine_tuning/jobs", &parameters).await?;

        let response: FineTuningJob = format_response(response.data)?;

        Ok(response)
    }

    /// List your organization's fine-tuning jobs.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<FineTuningJob>, APIError> {
        let response = self
            .client
            .get_with_query("/fine_tuning/jobs", &query)
            .await?;

        let response: ListResponse<FineTuningJob> = format_response(response)?;

        Ok(response)
    }

    /// Get info about a fine-tuning job.
    pub async fn retrieve(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let response = self.client.get(&format!("/fine_tuning/jobs/{id}")).await?;

        let response: FineTuningJob = format_response(response)?;

        Ok(response)
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let response = self
            .client
            .post(&format!("/fine_tuning/jobs/{id}/cancel"), &Value::Null)
            .await?;

        let response: FineTuningJob = format_response(response.data)?;

        Ok(response)
    }

    /// Get status updates for a fine-tuning job.
    pub async fn list_job_events(
        &self,
        id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<FineTuningJobEvent>, APIError> {
        let response = self
            .client
            .get_with_query(&format!("/fine_tuning/jobs/{id}/events"), &query)
            .await?;

        let response: ListResponse<FineTuningJobEvent> = format_response(response)?;

        Ok(response)
    }

    /// List checkpoints for a fine-tuning job.
    pub async fn list_checkpoints(
        &self,
        id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<FineTuningJobCheckpoint>, APIError> {
        let response = self
            .client
            .get_with_query(&format!("/fine_tuning/jobs/{id}/checkpoints"), &query)
            .await?;

        let response: ListResponse<FineTuningJobCheckpoint> = format_response(response)?;

        Ok(response)
    }
}
