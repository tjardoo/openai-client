use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UsageData<U> {
    pub object: String,
    pub start_time: u32,
    pub end_time: u32,
    pub results: Vec<U>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CompletionUsageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CompletionUsageParameters {
    /// Start time (Unix seconds) of the query time range, inclusive.
    pub start_time: u32,
    /// End time (Unix seconds) of the query time range, exclusive.
    pub end_time: Option<u32>,
    /// Width of each time bucket in response. By default, return 1 day.
    pub bucket_width: Option<BucketWidth>,
    /// Return only usage for these projects.
    pub project_ids: Option<Vec<String>>,
    /// Return only usage for these users.
    pub user_ids: Option<Vec<String>>,
    /// Return only usage for these API keys.
    pub api_key_ids: Option<Vec<String>>,
    /// Return only usage for these models.
    pub models: Option<Vec<String>>,
    /// If true, return batch jobs only. If false, return non-batch jobs only. By default, return both.
    pub batch: Option<bool>,
    /// Group the usage data by the specified fields.
    pub group_by: Option<GroupBy>,
    /// Specifies the number of buckets to return.
    pub limit: Option<u32>,
    /// A cursor for use in pagination. Corresponding to the next_page field from the previous response.
    pub page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CompletionUsage {
    pub object: String,
    /// The number of input tokens used.
    pub input_tokens: u32,
    /// The number of input tokens that has been cached from previous requests.
    pub input_cached_tokens: u32,
    /// The number of output tokens used.
    pub output_tokens: u32,
    /// The count of requests made to the model.
    pub num_model_requests: u32,
    /// When group_by=project_id, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When group_by=user_id, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When group_by=api_key_id, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When group_by=model, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
    /// When group_by=batch, this field tells whether the grouped usage result is batch or not.
    pub batch: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BucketWidth {
    #[serde(rename = "1m")]
    Minute,
    #[serde(rename = "1h")]
    Hour,
    #[serde(rename = "1d")]
    Day,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GroupBy {
    ProjectId,
    UserId,
    ApiKeyId,
    Model,
    Batch,
}
