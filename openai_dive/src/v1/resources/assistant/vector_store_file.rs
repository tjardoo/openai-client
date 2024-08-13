use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::v1::resources::shared::LastError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStoreFile {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always vector_store.file.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store file was created.
    pub created_at: u32,
    /// The total vector store usage in bytes. Note that this may be different from the original file size.
    pub usage_bytes: u64,
    /// The ID of the vector store that the File is attached to.
    pub vector_store_id: String,
    /// The status of the vector store file.
    pub status: VectorStoreFileStatus,
    /// The last error associated with this vector store file. Will be null if there are no errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    /// The strategy used to chunk the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<VectorStoreFileChunkingStrategy>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum VectorStoreFileChunkingStrategy {
    Auto,
    Static {
        r#static: VectorStoreFileStaticChunkingStrategy,
    },
    #[serde(rename = "other")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStoreFileStaticChunkingStrategy {
    /// The maximum number of tokens in each chunk. The default value is 800. The minimum value is 100 and the maximum value is 4096.
    pub max_chunk_size_tokens: u16,
    /// The number of tokens that overlap between chunks. The default value is 400.
    /// Note that the overlap must not exceed half of max_chunk_size_tokens.
    pub chunk_overlap_tokens: u16,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateVectorStoreFileParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateVectorStoreFileParameters {
    /// A File ID that the vector store should use.
    pub file_id: String,
    /// The chunking strategy used to chunk the file(s). If not set, will use the auto strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<VectorStoreFileChunkingStrategy>,
}
