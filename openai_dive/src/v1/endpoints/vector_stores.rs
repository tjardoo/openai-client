use crate::v1::{
    api::Client,
    error::APIError,
    helpers::format_response,
    resources::{
        shared::{DeletedObject, ListParameters, ListResponse},
        vector_store::{CreateVectorStoreParameters, ModifyVectorStoreParameters, VectorStore},
    },
};

pub struct VectorStores<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Vector stores are used to store files for use by the file_search tool.
    pub fn vector_stores(&self) -> VectorStores {
        VectorStores { client: self }
    }
}

impl VectorStores<'_> {
    /// Create a vector store.
    pub async fn create(
        &self,
        parameters: CreateVectorStoreParameters,
    ) -> Result<VectorStore, APIError> {
        let response = self.client.post("/vector_stores", &parameters).await?;

        let response: VectorStore = format_response(response.data)?;

        Ok(response)
    }

    /// Returns a list of vector stores.
    pub async fn list(
        &self,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<VectorStore>, APIError> {
        let response = self.client.get_with_query("/vector_stores", &query).await?;

        let response: ListResponse<VectorStore> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a vector store.
    pub async fn retrieve(&self, vector_store_id: &str) -> Result<VectorStore, APIError> {
        let response = self
            .client
            .get(&format!("/vector_stores/{vector_store_id}"))
            .await?;

        let response: VectorStore = format_response(response)?;

        Ok(response)
    }

    /// Modifies a vector store.
    pub async fn modify(
        &self,
        vector_store_id: &str,
        parameters: ModifyVectorStoreParameters,
    ) -> Result<VectorStore, APIError> {
        let response = self
            .client
            .post(&format!("/vector_stores/{vector_store_id}"), &parameters)
            .await?;

        let response: VectorStore = format_response(response.data)?;

        Ok(response)
    }

    /// Delete a vector store.
    pub async fn delete(&self, vector_store_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .client
            .delete(&format!("/threads/{vector_store_id}"))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
