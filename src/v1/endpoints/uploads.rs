use serde_json::Value;

use crate::v1::{
    api::Client,
    error::APIError,
    helpers::format_response,
    resources::upload::{
        AddPartParameters, CompleteUploadParameters, CreateUploadParameters, Upload, UploadPart,
    },
};

pub struct Uploads<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Allows you to upload large files in multiple parts.
    pub fn uploads(&self) -> Uploads {
        Uploads { client: self }
    }
}

impl Uploads<'_> {
    /// Creates an intermediate Upload object that you can add Parts to.
    pub async fn create(&self, parameters: CreateUploadParameters) -> Result<Upload, APIError> {
        let response = self.client.post("/uploads", &parameters).await?;

        let upload_response: Upload = format_response(response.data)?;

        Ok(upload_response)
    }

    /// Adds a Part to an Upload object. A Part represents a chunk of bytes from the file you are trying to upload.
    pub async fn add_part(
        &self,
        id: &str,
        parameters: AddPartParameters,
    ) -> Result<UploadPart, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let file_part = parameters.data.into_part().await?;
        form = form.part("data", file_part);

        let response = self
            .client
            .post_with_form(format!("/uploads/{id}/parts").as_str(), form)
            .await?;

        let upload_part_response: UploadPart = format_response(response)?;

        Ok(upload_part_response)
    }

    /// Completes the Upload.
    pub async fn complete(
        &self,
        id: &str,
        parameters: CompleteUploadParameters,
    ) -> Result<Upload, APIError> {
        let response = self
            .client
            .post(format!("/uploads/{id}/complete").as_str(), &parameters)
            .await?;

        let upload_response: Upload = format_response(response.data)?;

        Ok(upload_response)
    }

    /// Cancels the Upload.
    pub async fn cancel(&self, id: &str) -> Result<Upload, APIError> {
        let response = self
            .client
            .post(format!("/uploads/{id}/cancel").as_str(), &Value::Null)
            .await?;

        let upload_response: Upload = format_response(response.data)?;

        Ok(upload_response)
    }
}
