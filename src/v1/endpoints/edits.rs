use crate::v1::resources::edit::{EditParameters, EditResponse};
use crate::v1::{api::Client, error::APIError};
use serde_json::Value;

pub struct Edits<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn edits(&self) -> Edits {
        Edits {
            client: self,
        }
    }
}

impl Edits<'_> {
    pub async fn create(&self, parameters: EditParameters) -> Result<EditResponse, APIError> {
        let response = self.client.post("/edits", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let edit_response: EditResponse = serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(edit_response)
    }
}
