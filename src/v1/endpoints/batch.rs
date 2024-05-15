use crate::v1::{
    api::Client,
    error::APIError,
    helpers::format_response,
    resources::{
        batch::{Batch, CreateBatchParameters, ListBatchesResponse},
        shared::SimpleListParameters,
    },
};

pub struct Batches<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Create large batches of API requests for asynchronous processing.
    pub fn batches(&self) -> Batches {
        Batches { client: self }
    }
}

impl Batches<'_> {
    /// Creates and executes a batch from an uploaded file of requests
    pub async fn create(&self, parameters: CreateBatchParameters) -> Result<Batch, APIError> {
        let response = self.client.post("/batches", &parameters).await?;

        let batch_response: Batch = format_response(response.data)?;

        Ok(batch_response)
    }

    /// Retrieves a batch.
    pub async fn retrieve(&self, id: &str) -> Result<Batch, APIError> {
        let response = self.client.get(format!("/batches/{id}").as_str()).await?;

        let batch_response: Batch = format_response(response)?;

        Ok(batch_response)
    }

    /// Cancels an in-progress batch.
    pub async fn cancel(&self, id: &str) -> Result<Batch, APIError> {
        let response = self
            .client
            .post(format!("/batches/{id}/cancel").as_str(), &())
            .await?;

        let batch_response: Batch = format_response(response.data)?;

        Ok(batch_response)
    }

    /// List your organization's batches.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListBatchesResponse, APIError> {
        let response = self.client.get_with_query("/batches", &query).await?;

        let list_batches_response: ListBatchesResponse = format_response(response)?;

        Ok(list_batches_response)
    }
}
