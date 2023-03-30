use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct CompletionStreamParameters {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionStreamChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionStreamChoice {
    pub text: String,
    pub index: u32,
}
