use crate::v1::resources::shared::Usage;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "EmbeddingParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct EmbeddingParameters {
    /// Input text to embed, encoded as a string or array of tokens.
    /// To embed multiple inputs in a single request, pass an array of strings or array of token arrays.
    pub input: EmbeddingInput,
    /// ID of the model to use.
    pub model: String,
    /// The format to return the embeddings in. Can be either float or base64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// The number of dimensions the resulting output embeddings should have. Only supported in 'text-embedding-3' and later models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddingResponse {
    /// The object type, which is always "embedding".
    pub object: String,
    /// A list of embedding objects.
    pub data: Vec<Embedding>,
    /// The model used to generate the embeddings.
    pub model: String,
    /// Object containing usage information for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// The string that will be turned into an embedding.
    String(String),
    /// The array of strings that will be turned into an embedding.
    StringArray(Vec<String>),
    /// The array of integers that will be turned into an embedding.
    IntegerArray(Vec<u32>),
    /// The array of arrays containing integers that will be turned into an embedding.
    IntegerArrayArray(Vec<Vec<u32>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum EmbeddingOutput {
    Float(Vec<f64>),
    Base64(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub index: u32,
    /// The embedding vector, which is a list of floats. Or String when encoding format is set to 'base64'.
    pub embedding: EmbeddingOutput,
    /// The object type, which is always "embedding".
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}

impl Default for EmbeddingInput {
    fn default() -> Self {
        EmbeddingInput::String(String::new())
    }
}
