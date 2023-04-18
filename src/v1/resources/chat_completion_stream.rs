use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<DeltaField>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaField {
    pub delta: DeltaValue,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}