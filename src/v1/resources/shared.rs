use serde::{Deserialize, Serialize};

#[cfg(feature = "download")]
use rand::{distributions::Alphanumeric, Rng};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BaseModel {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    pub prompt_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FinishReason {
    /// API returned complete message, or a message terminated by one of the stop sequences provided via the stop parameter.
    #[serde(rename(deserialize = "stop"))]
    StopSequenceReached,
    /// Incomplete model output due to max_tokens parameter or token limit.
    #[serde(rename(deserialize = "length"))]
    TokenLimitReached,
    /// Omitted content due to a flag from our content filters.
    #[serde(rename(deserialize = "content_filter"))]
    ContentFilterFlagged,
    /// The model decided to call a function.
    #[serde(rename(deserialize = "function_call"))]
    FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum StopToken {
    String(String),
    Array(Vec<String>),
}

#[cfg(feature = "download")]
pub fn generate_file_name(path: &str, length: u32, file_type: &str) -> String {
    let random_file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();

    format!("{}/{}.{}", path, random_file_name, file_type)
}
