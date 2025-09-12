use crate::v1::{
    api::Client,
    error::APIError,
    helpers::format_response,
    resources::{
        shared::{DeletedObject, ListParameters, ListResponse},
        vector_store_file::{CreateVectorStoreFileParameters, VectorStoreFile},
    },
};

pub struct VectorStoreFiles<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Vector store files represent files inside a vector store.
    pub fn vector_store_files(&self) -> VectorStoreFiles<'_> {
        VectorStoreFiles { client: self }
    }
}

impl VectorStoreFiles<'_> {
    /// Create a vector store file by attaching a File to a vector store.
    pub async fn create(
        &self,
        vector_store_id: &str,
        parameters: CreateVectorStoreFileParameters,
    ) -> Result<VectorStoreFile, APIError> {
        let response = self
            .client
            .post(
                &format!("/vector_stores/{vector_store_id}/files"),
                &parameters,
                None,
            )
            .await?;

        let response: VectorStoreFile = format_response(response.data)?;

        Ok(response)
    }

    /// Returns a list of vector store files.
    pub async fn list(
        &self,
        vector_store_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<VectorStoreFile>, APIError> {
        let response = self
            .client
            .get_with_query(&format!("/vector_stores/{vector_store_id}/files"), &query)
            .await?;

        let response: ListResponse<VectorStoreFile> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a vector store.
    pub async fn retrieve(
        &self,
        vector_store_id: &str,
        vector_store_file_id: &str,
    ) -> Result<VectorStoreFile, APIError> {
        let response = self
            .client
            .get(&format!(
                "/vector_stores/{vector_store_id}/files/{vector_store_file_id}"
            ))
            .await?;

        let response: VectorStoreFile = format_response(response)?;

        Ok(response)
    }

    /// Delete a vector store file.
    pub async fn delete(
        &self,
        vector_store_id: &str,
        vector_store_file_id: &str,
    ) -> Result<DeletedObject, APIError> {
        let response = self
            .client
            .delete(&format!(
                "/vector_stores/{vector_store_id}/files/{vector_store_file_id}"
            ))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
