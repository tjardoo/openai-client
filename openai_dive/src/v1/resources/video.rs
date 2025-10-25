use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::v1::resources::response::response::ResponseError;

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateVideoParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateVideoParameters {
    /// Text prompt that describes the video to generate.
    pub prompt: String,
    /// Optional image reference that guides generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_reference: Option<String>,
    /// The video generation model to use. Defaults to `sora-2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Clip duration in seconds. Defaults to `4 seconds`. Supported values are 4, 8 and 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<String>,
    /// Output resolution formatted as width x height. Defaults to `720x1280`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<VideoSize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CreateVideoRemixParameters {
    /// Updated text prompt that directs the remix generation.
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VideoJob {
    /// Unix timestamp (seconds) for when the job completed, if finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u32>,
    /// Unix timestamp (seconds) for when the job was created.
    pub created_at: u32,
    /// Error payload that explains why generation failed, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    /// Unix timestamp (seconds) for when the downloadable assets expire, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u32>,
    /// Unique identifier for the video job.
    pub id: String,
    /// The video generation model that produced the job.
    pub model: String,
    /// The object type, which is always `video`.
    pub object: String,
    /// Approximate completion percentage for the generation task.
    pub progress: u8,
    /// Identifier of the source video if this video is a remix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixed_from_video_id: Option<String>,
    /// Duration of the generated clip in seconds.
    pub seconds: String,
    /// The resolution of the generated video.
    pub size: VideoSize,
    /// Current lifecycle status of the video job.
    pub status: VideoJobStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VideoJobStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VideoSize {
    #[serde(rename = "720x1280")]
    Size720x1280,
    #[serde(rename = "1280x720")]
    Size1280x720,
    #[serde(rename = "1024x1792")]
    Size1024x1792,
    #[serde(rename = "1792x1024")]
    Size1792x1024,
}
