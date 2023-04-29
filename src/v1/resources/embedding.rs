use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone)]
pub struct EmbeddingParameters {
    pub model: String,
    pub input: String,
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
