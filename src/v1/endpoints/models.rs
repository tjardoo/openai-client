use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::model::{ListModelResponse, Model};
use serde_json::Value;

pub struct Models<'a> {
    pub client: &'a Client,
}

impl Client {
    /// List and describe the various models available in the API.
    pub fn models(&self) -> Models {
        Models { client: self }
    }
}

impl Models<'_> {
    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub async fn list(&self) -> Result<ListModelResponse, APIError> {
        let response = self.client.get("/models").await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let list_model_response: ListModelResponse = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_model_response)
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub async fn get(&self, model_id: &str) -> Result<Model, APIError> {
        let path = format!("/models/{}", model_id);

        let response = self.client.get(&path).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let model_response: Model = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(model_response)
    }

    /// Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.
    pub async fn delete(&self, model_id: &str) -> Result<Model, APIError> {
        let path = format!("/models/{}", model_id);

        let response = self.client.delete(&path).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let model_response: Model = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(model_response)
    }
}
