use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseModel {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FinishReason {
    #[serde(rename(deserialize = "length"))]
    TokenLimitReached,
    #[serde(rename(deserialize = "stop"))]
    StopSequenceReached,
}