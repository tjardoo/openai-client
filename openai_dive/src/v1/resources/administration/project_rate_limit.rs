use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectRateLimit {
    /// The object type, which is always 'project.rate_limit'.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The model this rate limit applies to.
    pub model: String,
    /// The maximum requests per minute.
    pub max_requests_per_1_minute: u32,
    /// The maximum tokens per minute.
    pub max_tokens_per_1_minute: u32,
    /// The maximum images per minute. Only present for relevant models.
    pub max_images_per_1_minute: Option<u32>,
    /// The maximum audio megabytes per minute. Only present for relevant models.
    pub max_audio_megabytes_per_1_minute: Option<u32>,
    /// The maximum requests per day. Only present for relevant models.
    pub max_requests_per_1_day: Option<u32>,
    /// The maximum batch input tokens per day. Only present for relevant models.
    pub batch_1_day_max_input_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ModifyProjectRateLimitParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModifyProjectRateLimitParameters {
    /// The maximum requests per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_minute: Option<u32>,
    /// The maximum tokens per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens_per_1_minute: Option<u32>,
    /// The maximum images per minute. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images_per_1_minute: Option<u32>,
    /// The maximum audio megabytes per minute. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_megabytes_per_1_minute: Option<u32>,
    /// The maximum requests per day. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_day: Option<u32>,
    /// The maximum batch input tokens per day. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_1_day_max_input_tokens: Option<u32>,
}
