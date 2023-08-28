use serde::{Serialize, Deserialize};

#[deprecated(since = "0.2.12")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionStreamChoice>,
}

#[deprecated(since = "0.2.12")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionStreamChoice {
    pub text: String,
    pub index: u32,
}
