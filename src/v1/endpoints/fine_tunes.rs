use crate::v1::resources::fine_tune::{CreateFineTuneParameters, FineTune};
use crate::v1::{api::Client, error::APIError};
use serde_json::Value;

pub struct FineTunes<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn fine_tunes(&self) -> FineTunes {
        FineTunes {
            client: self,
        }
    }
}

impl FineTunes<'_> {
    pub async fn create(&self, parameters: CreateFineTuneParameters) -> Result<FineTune, APIError> {
        let response = self.client.post("/fine-tunes", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tune_response: FineTune = serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tune_response)
    }

    pub async fn list(&self) -> Result<Vec<FineTune>, APIError> {
        let response = self.client.get("/fine-tunes").await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tunes: Vec<FineTune> = serde_json::from_value(value["data"].clone()).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tunes)
    }
}
