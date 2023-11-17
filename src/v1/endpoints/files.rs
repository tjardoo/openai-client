use crate::v1::api::file_from_disk_to_form_part;
use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::file::ListFilesParameters;
use crate::v1::resources::file::ListFilesResponse;
use crate::v1::resources::file::{DeletedFile, File, UploadFileParameters};
use serde_json::Value;

pub struct Files<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
    pub fn files(&self) -> Files {
        Files { client: self }
    }
}

impl Files<'_> {
    /// Returns a list of files that belong to the user's organization.
    pub async fn list(
        &self,
        query: Option<ListFilesParameters>,
    ) -> Result<ListFilesResponse, APIError> {
        let response = self.client.get_with_query("/files", &query).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let files: ListFilesResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(files)
    }

    /// Upload a file that can be used across various endpoints.
    pub async fn upload(&self, parameters: UploadFileParameters) -> Result<File, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let file = file_from_disk_to_form_part(parameters.file).await?;
        form = form.part("file", file);

        form = form.text("purpose", parameters.purpose.to_string());

        let response = self.client.post_with_form("/files", form).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let file_response: File = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(file_response)
    }

    /// Delete a file.
    pub async fn delete(&self, id: &str) -> Result<DeletedFile, APIError> {
        let response = self.client.delete(format!("/files/{id}").as_str()).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let deleted_file_response: DeletedFile = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(deleted_file_response)
    }

    /// Returns information about a specific file.
    pub async fn retrieve(&self, id: &str) -> Result<File, APIError> {
        let response = self.client.get(format!("/files/{id}").as_str()).await?;

        let value: Value = serde_json::from_str(&response).unwrap();

        let file_response: File = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(file_response)
    }

    /// Returns the contents of the specified file.
    pub async fn retrieve_content(&self, id: &str) -> Result<String, APIError> {
        let response = self
            .client
            .get(format!("/files/{id}/content").as_str())
            .await?;

        Ok(response)
    }
}
