use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::assistant::assistant::Assistant;
use crate::v1::resources::assistant::assistant::AssistantParameters;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListParameters;
use crate::v1::resources::shared::ListResponse;

pub struct Assistants<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Build assistants that can call models and use tools to perform tasks.
    pub fn assistants(&self) -> Assistants {
        Assistants { client: self }
    }
}

impl Assistants<'_> {
    /// Create an assistant with a model and instructions.
    pub async fn create(&self, parameters: AssistantParameters) -> Result<Assistant, APIError> {
        let response = self.client.post("/assistants", &parameters).await?;

        let assistant_response: Assistant = format_response(response.data)?;

        Ok(assistant_response)
    }

    /// Retrieves an assistant.
    pub async fn retrieve(&self, id: &str) -> Result<Assistant, APIError> {
        let response = self.client.get(&format!("/assistants/{id}")).await?;

        let assistant_response: Assistant = format_response(response)?;

        Ok(assistant_response)
    }

    /// Modifies an assistant.
    pub async fn modify(
        &self,
        id: &str,
        parameters: AssistantParameters,
    ) -> Result<Assistant, APIError> {
        let response = self
            .client
            .post(&format!("/assistants/{id}"), &parameters)
            .await?;

        let assistant_response: Assistant = format_response(response.data)?;

        Ok(assistant_response)
    }

    /// Delete an assistant.
    pub async fn delete(&self, id: &str) -> Result<DeletedObject, APIError> {
        let response = self.client.delete(&format!("/assistants/{id}")).await?;

        let deleted_object: DeletedObject = format_response(response)?;

        Ok(deleted_object)
    }

    /// Returns a list of assistants.
    pub async fn list(
        &self,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<Assistant>, APIError> {
        let response = self.client.get_with_query("/assistants", &query).await?;

        let list_assistants_response: ListResponse<Assistant> = format_response(response)?;

        Ok(list_assistants_response)
    }
}
