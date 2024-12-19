use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UsageBucket<U> {
    /// The object type, which is always "bucket".
    pub object: String,
    /// Start time (Unix seconds) of the query time range, inclusive.
    pub start_time: u32,
    /// End time (Unix seconds) of the query time range, exclusive.
    pub end_time: u32,
    /// The usage data for this bucket.
    pub results: Vec<U>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "UsageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct UsageParameters {
    /// Start time (Unix seconds) of the query time range, inclusive.
    pub start_time: u32,
    /// End time (Unix seconds) of the query time range, exclusive.
    pub end_time: Option<u32>,
    /// Width of each time bucket in response. By default, return 1 day.
    pub bucket_width: Option<BucketWidth>,
    /// Return only usages for these sources.
    pub sources: Option<Vec<ImageSource>>,
    /// Return only usages for these image sizes.
    pub sizes: Option<Vec<ImageSize>>,
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
    pub group_by: Option<Vec<GroupBy>>,
    /// Specifies the number of buckets to return.
    pub limit: Option<u32>,
    /// A cursor for use in pagination. Corresponding to the next_page field from the previous response.
    pub page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CompletionUsage {
    /// The object type, which is always "organization.usage.completions.result".
    pub object: String,
    /// The aggregated number of text input tokens used, including cached tokens.
    pub input_tokens: u32,
    /// The aggregated number of text input tokens that has been cached from previous requests.
    pub input_cached_tokens: u32,
    /// The aggregated number of text output tokens used.
    pub output_tokens: u32,
    /// The aggregated number of audio input tokens used, including cached tokens.
    pub input_audio_tokens: Option<u32>,
    /// The aggregated number of audio output tokens used.
    pub output_audio_tokens: Option<u32>,
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
pub struct EmbeddingUsage {
    /// The object type, which is always "organization.usage.embeddings.result".
    pub object: String,
    /// The aggregated number of input tokens used.
    pub input_tokens: u32,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModerationUsage {
    /// The object type, which is always "organization.usage.moderations.result".
    pub object: String,
    /// The aggregated number of input tokens used.
    pub input_tokens: u32,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUsage {
    /// The object type, which is always "organization.usage.images.result".
    pub object: String,
    /// The number of images processed.
    pub images: u32,
    /// The count of requests made to the model.
    pub num_model_requests: u32,
    /// When group_by=source, this field provides the source of the grouped usage result.
    pub source: Option<ImageSource>,
    /// When group_by=size, this field provides the image size of the grouped usage result.
    pub size: Option<ImageSize>,
    /// When group_by=project_id, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When group_by=user_id, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When group_by=api_key_id, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When group_by=model, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioSpeechUsage {
    /// The object type, which is always "organization.usage.audio_speeches.result".
    pub object: String,
    /// The number of characters processed.
    pub characters: u32,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioTranscriptionUsage {
    /// The object type, which is always "organization.usage.audio_transcriptions.result".
    pub object: String,
    /// The number of seconds processed.
    pub seconds: u32,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VectorStoreUsage {
    /// The object type, which is always "organization.usage.vector_stores.result".
    pub object: String,
    /// The vector stores usage in bytes.
    pub usage_bytes: u32,
    /// When group_by=project_id, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeInterpreterSessionUsage {
    /// The object type, which is always "organization.usage.code_interpreter_sessions.result".
    pub object: String,
    /// The number of code interpreter sessions.
    pub sessions: u32,
    /// When group_by=project_id, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CostUsage {
    /// The object type, which is always "organization.usage.code_interpreter_sessions.result".
    pub object: String,
    /// The monetary value in its associated currency.
    pub amount: Amount,
    /// When group_by=line_item, this field provides the line item of the grouped costs result.
    pub line_item: Option<String>,
    /// When group_by=project_id, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
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
    LineItem,
    Size,
    Source,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Amount {
    /// The numeric value of the cost.
    pub value: f32,
    /// Lowercase ISO-4217 currency e.g. "usd"
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImageSource {
    #[serde(rename = "image.generation")]
    ImageGeneration,
    #[serde(rename = "unsplimage.editash")]
    ImageEdit,
    #[serde(rename = "image.variation")]
    ImageVariation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Size256X256,
    #[serde(rename = "512x512")]
    Size512X512,
    #[serde(rename = "1024x1024")]
    Size1024X1024,
    #[serde(rename = "1792x1792")]
    Size1792X1792,
    #[serde(rename = "1024x1792")]
    Size1024X1792,
}
