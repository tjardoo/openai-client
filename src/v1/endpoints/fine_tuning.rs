use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::fine_tuning::{
    CreateFineTuningParameters, FineTuningJob, FineTuningJobEvent,
};
use serde_json::Value;

pub struct FineTuning<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn fine_tuning(&self) -> FineTuning {
        FineTuning { client: self }
    }
}

impl FineTuning<'_> {
    pub async fn create(
        &self,
        parameters: CreateFineTuningParameters,
    ) -> Result<FineTuningJob, APIError> {
        let response = self.client.post("/fine-tuning/jobs", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tuning_response: FineTuningJob = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tuning_response)
    }

    pub async fn list(&self) -> Result<Vec<FineTuningJob>, APIError> {
        let response = self.client.get("/fine_tuning/jobs").await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tunings: Vec<FineTuningJob> = serde_json::from_value(value["data"].clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tunings)
    }

    pub async fn retrieve(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let response = self
            .client
            .get(format!("/fine_tuning/jobs/{id}").as_str())
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tuning_response: FineTuningJob = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tuning_response)
    }

    pub async fn cancel(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let parameters = {
            struct CancelFineTuningParameters {}
        };

        let response = self
            .client
            .post(
                format!("/fine-tuning/jobs/{id}/cancel").as_str(),
                &parameters,
            )
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tuning_response: FineTuningJob = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tuning_response)
    }

    pub async fn list_events(&self, id: &str) -> Result<Vec<FineTuningJobEvent>, APIError> {
        let response = self
            .client
            .get(format!("/fine-tuning/jobs/{id}/events").as_str())
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tuning_response: Vec<FineTuningJobEvent> =
            serde_json::from_value(value["data"].clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tuning_response)
    }
}
