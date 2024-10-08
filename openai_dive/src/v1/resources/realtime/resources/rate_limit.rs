use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RateLimit {
    /// The name of the rate limit ("requests", "tokens", "input_tokens", "output_tokens").
    pub name: RateLimitName,
    /// The maximum allowed value for the rate limit.
    pub limit: u32,
    /// The remaining value before the limit is reached.
    pub remaining: u32,
    /// Seconds until the rate limit resets.
    pub reset_seconds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitName {
    Requests,
    Tokens,
    InputTokens,
    OutputTokens,
}
