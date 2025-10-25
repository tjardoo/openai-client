use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::video::{CreateVideoParameters, VideoJob};

pub struct Videos<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Generate videos.
    pub fn videos(&self) -> Videos<'_> {
        Videos { client: self }
    }
}

impl Videos<'_> {
    /// Create a video
    pub async fn create(&self, parameters: CreateVideoParameters) -> Result<VideoJob, APIError> {
        let response = self.client.post("/videos", &parameters, None).await?;

        let video_job: VideoJob = format_response(response.data)?;

        Ok(video_job)
    }

    /// Retrieve a video
    pub async fn retrieve(&self, video_id: &str) -> Result<VideoJob, APIError> {
        let response = self.client.get(&format!("/videos/{video_id}")).await?;

        let video_job: VideoJob = format_response(response)?;

        Ok(video_job)
    }

    // @todo remix video

    // @todo list video

    // @todo delete video

    // @todo retrieve video content
}
