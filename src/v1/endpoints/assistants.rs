use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::Assistant;
use crate::v1::resources::assistant::CreateAssistantParameters;
use crate::v1::resources::assistant::ListAssistantsParameters;
use crate::v1::resources::assistant::ListAssistantsResponse;

pub struct Assistants<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Returns a list of assistants.
    pub fn assistants(&self) -> Assistants {
        Assistants { client: self }
    }
}

impl Assistants<'_> {
    /// Create an assistant with a model and instructions.
    pub async fn create(
        &self,
        parameters: CreateAssistantParameters,
    ) -> Result<Assistant, APIError> {
        let response = self.client.post("/assistants", &parameters).await?;

        let value = validate_request(response).await?;

        let assistant_response: Assistant = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_response)
    }

    /// Retrieves an assistant.
    pub async fn retrieve(&self, id: &str) -> Result<Assistant, APIError> {
        let response = self
            .client
            .get(format!("/assistants/{id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let assistant_response: Assistant = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_response)
    }

    /// Returns a list of assistants.
    pub async fn list(
        &self,
        query: Option<ListAssistantsParameters>,
    ) -> Result<ListAssistantsResponse, APIError> {
        let response = self.client.get_with_query("/assistants", &query).await?;

        let value = validate_request(response).await?;

        let list_assistants_response: ListAssistantsResponse =
            serde_json::from_value(value.clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_assistants_response)
    }
}