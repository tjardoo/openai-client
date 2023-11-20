use crate::v1::resources::shared::Usage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddingParameters {
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays.
    pub input: String,
    /// ID of the model to use.
    pub model: String,
    /// The format to return the embeddings in. Can be either float or base64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub index: u32,
    /// The embedding vector, which is a list of floats.
    pub embedding: Vec<f64>,
    /// The object type, which is always "embedding".
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}
