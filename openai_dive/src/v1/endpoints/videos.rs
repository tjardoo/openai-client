use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::shared::{DeletedObject, ListResponse, SimpleListParameters};
use crate::v1::resources::video::{CreateVideoParameters, CreateVideoRemixParameters, VideoJob};

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

    /// Create a video remix
    pub async fn create_remix(
        &self,
        video_id: &str,
        parameters: CreateVideoRemixParameters,
    ) -> Result<VideoJob, APIError> {
        let response = self
            .client
            .post(&format!("/videos/{video_id}/remix"), &parameters, None)
            .await?;

        let video_job: VideoJob = format_response(response.data)?;

        Ok(video_job)
    }

    /// Retrieve a video
    pub async fn retrieve(&self, video_id: &str) -> Result<VideoJob, APIError> {
        let response = self.client.get(&format!("/videos/{video_id}")).await?;

        let video_job: VideoJob = format_response(response)?;

        Ok(video_job)
    }

    /// List videos
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<VideoJob>, APIError> {
        let response = self.client.get_with_query("/videos", &query).await?;

        let response: ListResponse<VideoJob> = format_response(response)?;

        Ok(response)
    }

    /// Delete a video
    pub async fn delete(&self, id: &str) -> Result<DeletedObject, APIError> {
        let response = self.client.delete(&format!("/videos/{id}")).await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }

    /// Download video content
    pub async fn retrieve_content(&self, id: &str) -> Result<Vec<u8>, APIError> {
        let response = self
            .client
            .get_raw(&format!("/videos/{id}/content"))
            .await?;

        Ok(response.to_vec())
    }
}
