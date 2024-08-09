use crate::v1::{
    endpoints::assistants::assistants::Assistants,
    error::APIError,
    helpers::format_response,
    resources::{
        assistant::vector_store_file::{CreateVectorStoreFileParameters, VectorStoreFile},
        shared::{DeletedObject, ListParameters, ListResponse},
    },
};

pub struct VectorStoreFiles<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Vector store files represent files inside a vector store.
    pub fn vector_store_files(&self) -> VectorStoreFiles {
        VectorStoreFiles { assistant: self }
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
            .assistant
            .client
            .post(
                &format!("/vector_stores/{vector_store_id}/files"),
                &parameters,
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
            .assistant
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
            .assistant
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
            .assistant
            .client
            .delete(&format!(
                "/vector_stores/{vector_store_id}/files/{vector_store_file_id}"
            ))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
