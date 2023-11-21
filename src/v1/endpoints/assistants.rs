use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::ListAssistantsParameters;
use crate::v1::resources::assistant::ListAssistantsResponse;

pub struct Assistant<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Returns a list of assistants.
    pub fn assistants(&self) -> Assistant {
        Assistant { client: self }
    }
}

impl Assistant<'_> {
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
