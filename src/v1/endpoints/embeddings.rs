use serde_json::Value;

use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::embedding::{EmbeddingParameters, EmbeddingResponse};

pub struct Embeddings<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn embeddings(&self) -> Embeddings {
        Embeddings { client: self }
    }
}

impl Embeddings<'_> {
    pub async fn create(
        &self,
        parameters: EmbeddingParameters,
    ) -> Result<EmbeddingResponse, APIError> {
        let response = self.client.post("/embeddings", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let embedding_response: EmbeddingResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(embedding_response)
    }
}
