use crate::v1::endpoints::assistants::Assistants;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::AssistantFile;
use crate::v1::resources::assistant::CreateAssistantFileParameters;
use crate::v1::resources::assistant::ListAssistantFilesParameters;
use crate::v1::resources::assistant::ListAssistantFilesResponse;
use crate::v1::resources::shared::DeletedObject;

pub struct Files<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Files attached to an assistant.
    pub fn files(&self) -> Files {
        Files { assistant: self }
    }
}

impl Files<'_> {
    /// Create an assistant file by attaching a file to an assistant.
    pub async fn create(
        &self,
        id: &str,
        parameters: CreateAssistantFileParameters,
    ) -> Result<AssistantFile, APIError> {
        let response = self
            .assistant
            .client
            .post(format!("/assistants/{id}/files").as_str(), &parameters)
            .await?;

        let value = validate_request(response).await?;

        let assistant_file_response: AssistantFile = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_file_response)
    }

    /// Retrieves an assistant file.
    pub async fn retrieve(&self, id: &str, file_id: &str) -> Result<AssistantFile, APIError> {
        let response = self
            .assistant
            .client
            .get(format!("/assistants/{id}/files/{file_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let assistant_file_response: AssistantFile = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(assistant_file_response)
    }

    /// Delete an assistant file.
    pub async fn delete(&self, id: &str, file_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .assistant
            .client
            .delete(format!("/assistants/{id}/files/{file_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let deleted_object: DeletedObject = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(deleted_object)
    }

    /// Returns a list of assistant files.
    pub async fn list(
        &self,
        id: &str,
        query: Option<ListAssistantFilesParameters>,
    ) -> Result<ListAssistantFilesResponse, APIError> {
        let response = self
            .assistant
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
