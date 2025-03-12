use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::vector_store_file::VectorStoreFileChunkingStrategy;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStore {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always vector_store.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store was created.
    pub created_at: u32,
    /// The name of the vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The total number of bytes used by the files in the vector store.
    pub usage_bytes: u64,
    /// The number of files in the vector store per status.
    pub file_counts: VectorStoreFileCounts,
    /// The status of the vector store.
    pub status: VectorStoreStatus,
    /// The expiration policy for a vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    /// The Unix timestamp (in seconds) for when the vector store will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the vector store was last active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<u32>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStoreFileCounts {
    /// The number of files that are currently being processed.
    pub in_progress: u32,
    /// The number of files that have been successfully processed.
    pub completed: u32,
    /// The number of files that have failed to process.
    pub failed: u32,
    /// The number of files that were cancelled.
    pub cancelled: u32,
    /// The total number of files.
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreStatus {
    InProgress,
    Completed,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStoreExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies.
    pub anchor: String,
    /// The number of days after the anchor time that the vector store will expire.
    pub days: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateVectorStoreParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateVectorStoreParameters {
    /// A list of File IDs that the vector store should use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// The name of the vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The expiration policy for a vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    /// The strategy used to chunk the file.
    pub chunking_strategy: Option<VectorStoreFileChunkingStrategy>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ModifyVectorStoreParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModifyVectorStoreParameters {
    /// The name of the vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The expiration policy for a vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
