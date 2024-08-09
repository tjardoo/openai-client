use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::file::ListFilesParameters;
use crate::v1::resources::file::ListFilesResponse;
use crate::v1::resources::file::{File, UploadFileParameters};
use crate::v1::resources::shared::DeletedObject;

pub struct Files<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Files are used to upload documents that can be used with features like Assistants, Fine-tuning, and Batch API.
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

        let response: ListFilesResponse = format_response(response)?;

        Ok(response)
    }

    /// Upload a file that can be used across various endpoints.
    pub async fn upload(&self, parameters: UploadFileParameters) -> Result<File, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let file = parameters.file.into_part().await?;
        form = form.part("file", file);

        form = form.text("purpose", parameters.purpose.to_string());

        let response = self.client.post_with_form("/files", form).await?;

        let response: File = format_response(response)?;

        Ok(response)
    }

    /// Delete a file.
    pub async fn delete(&self, id: &str) -> Result<DeletedObject, APIError> {
        let response = self.client.delete(&format!("/files/{id}")).await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }

    /// Returns information about a specific file.
    pub async fn retrieve(&self, id: &str) -> Result<File, APIError> {
        let response = self.client.get(&format!("/files/{id}")).await?;

        let response: File = format_response(response)?;

        Ok(response)
    }

    /// Returns the contents of the specified file.
    pub async fn retrieve_content(&self, id: &str) -> Result<String, APIError> {
        let response = self.client.get(&format!("/files/{id}/content")).await?;

        Ok(response)
    }
}
