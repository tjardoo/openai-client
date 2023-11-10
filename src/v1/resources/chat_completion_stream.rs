use serde::{Serialize, Deserialize};
use crate::v1::resources::chat_completion::Role;
use crate::v1::resources::chat_completion::FunctionCall;
use crate::v1::resources::shared::FinishReason;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<DeltaField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeltaField {
    pub delta: DeltaValue,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub finish_reason: Option<FinishReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeltaValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<Role>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function_call: Option<FunctionCall>,
}
