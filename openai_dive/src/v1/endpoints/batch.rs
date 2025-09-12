use crate::v1::{
    api::Client,
    error::APIError,
    helpers::format_response,
    resources::{
        batch::{Batch, CreateBatchParameters},
        shared::{ListResponse, SimpleListParameters},
    },
};

pub struct Batches<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Create large batches of API requests for asynchronous processing.
    pub fn batches(&self) -> Batches<'_> {
        Batches { client: self }
    }
}

impl Batches<'_> {
    /// Creates and executes a batch from an uploaded file of requests
    pub async fn create(&self, parameters: CreateBatchParameters) -> Result<Batch, APIError> {
        let response = self.client.post("/batches", &parameters, None).await?;

        let response: Batch = format_response(response.data)?;

        Ok(response)
    }

    /// Retrieves a batch.
    pub async fn retrieve(&self, id: &str) -> Result<Batch, APIError> {
        let response = self.client.get(&format!("/batches/{id}")).await?;

        let response: Batch = format_response(response)?;

        Ok(response)
    }

    /// Cancels an in-progress batch.
    pub async fn cancel(&self, id: &str) -> Result<Batch, APIError> {
        let response = self
            .client
            .post(&format!("/batches/{id}/cancel"), &(), None)
            .await?;

        let response: Batch = format_response(response.data)?;

        Ok(response)
    }

    /// List your organization's batches.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<Batch>, APIError> {
        let response = self.client.get_with_query("/batches", &query).await?;

        let response: ListResponse<Batch> = format_response(response)?;

        Ok(response)
    }
}
