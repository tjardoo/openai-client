use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::Assistant;
use crate::v1::resources::assistant::AssistantFile;
use crate::v1::resources::assistant::AssistantParameters;
use crate::v1::resources::assistant::CreateAssistantFileParameters;
use crate::v1::resources::assistant::ListAssistantFilesParameters;
use crate::v1::resources::assistant::ListAssistantFilesResponse;
use crate::v1::resources::assistant::ListAssistantsParameters;
use crate::v1::resources::assistant::ListAssistantsResponse;
use crate::v1::resources::shared::DeletedObject;

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

    /// Modifies an assistant.
    pub async fn modify(
        &self,
        id: &str,
        parameters: AssistantParameters,
    ) -> Result<Assistant, APIError> {
        let response = self
            .client
            .post(format!("/assistants/{id}").as_str(), &parameters)
            .await?;

        let value = validate_request(response).await?;

        let assistant_response: Assistant = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_response)
    }

    /// Delete an assistant.
    pub async fn delete(&self, id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .client
            .delete(format!("/assistants/{id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let deleted_object: DeletedObject = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(deleted_object)
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

    /// Create an assistant file by attaching a file to an assistant.
    pub async fn create_file(
        &self,
        id: &str,
        parameters: CreateAssistantFileParameters,
    ) -> Result<AssistantFile, APIError> {
        let response = self
            .client
            .post(format!("/assistants/{id}/files").as_str(), &parameters)
            .await?;

        let value = validate_request(response).await?;

        let assistant_file_response: AssistantFile = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_file_response)
    }

    /// Retrieves an assistant file.
    pub async fn retrieve_file(&self, id: &str, file_id: &str) -> Result<AssistantFile, APIError> {
        let response = self
            .client
            .get(format!("/assistants/{id}/files/{file_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let assistant_file_response: AssistantFile = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_file_response)
    }

    /// Delete an assistant file.
    pub async fn delete_file(&self, id: &str, file_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .client
            .delete(format!("/assistants/{id}/files/{file_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let deleted_object: DeletedObject = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(deleted_object)
    }

    /// Returns a list of assistant files.
    pub async fn list_files(
        &self,
        id: &str,
        query: Option<ListAssistantFilesParameters>,
    ) -> Result<ListAssistantFilesResponse, APIError> {
        let response = self
            .client
            .get_with_query(format!("/assistants/{id}/files").as_str(), &query)
            .await?;

        let value = validate_request(response).await?;

        let list_assistant_files_response: ListAssistantFilesResponse =
            serde_json::from_value(value.clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_assistant_files_response)
    }
}
