use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::embedding::{EmbeddingParameters, EmbeddingResponse};

pub struct Embeddings<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
    pub fn embeddings(&self) -> Embeddings {
        Embeddings { client: self }
    }
}

impl Embeddings<'_> {
    /// Creates an embedding vector representing the input text.
    pub async fn create(
        &self,
        parameters: EmbeddingParameters,
    ) -> Result<EmbeddingResponse, APIError> {
        let response = self.client.post("/embeddings", &parameters).await?;

        let embedding_response: EmbeddingResponse = format_response(response.data)?;

        Ok(embedding_response)
    }
}
