use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::v1::resources::shared::{Usage, FinishReason, StopToken};

#[derive(Serialize, Debug, Clone)]
pub struct SimpleCompletionParameters {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    pub max_tokens: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct CompletionParameters {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: FinishReason,
}
