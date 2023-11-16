use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embedding {
    pub object: String,
    pub embedding: Vec<f64>,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}
