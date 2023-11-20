use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
    /// Number of tokens in the entire response.
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
