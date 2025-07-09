use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::response::request::ResponseParameters;
use crate::v1::resources::response::response::ResponseObject;
use crate::v1::resources::shared::DeletedObject;

pub struct Responses<'a> {
    pub client: &'a Client,
}

impl Client {
    /// OpenAI's most advanced interface for generating model responses. Supports text and image inputs, and text outputs.
    pub fn responses(&self) -> Responses {
        Responses { client: self }
    }
}

impl Responses<'_> {
    /// Creates a model response.
    pub async fn create(&self, parameters: ResponseParameters) -> Result<ResponseObject, APIError> {
        let response = self.client.post("/responses", &parameters).await?;

        let response: ResponseObject = format_response(response.data)?;

        Ok(response)
    }

    /// Retrieves a model response with the given ID.
    pub async fn retrieve(&self, response_id: &str) -> Result<ResponseObject, APIError> {
        let response = self
            .client
            .get(&format!("/responses/{response_id}"))
            .await?;

        let response: ResponseObject = format_response(response)?;

        Ok(response)
    }

    /// Deletes a model response with the given ID.
    pub async fn delete(&self, response_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .client
            .delete(&format!("/responses/{response_id}"))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
