use serde::{Serialize, Deserialize};
use super::shared::{Usage, FinishReason};

#[derive(Serialize, Debug)]
pub struct CompletionParameters {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

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
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: FinishReason,
}
