use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::moderation::{ModerationParameters, ModerationResponse};

pub struct Moderations<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Given some input text, outputs if the model classifies it as potentially harmful across several categories.
    pub fn moderations(&self) -> Moderations {
        Moderations { client: self }
    }
}

impl Moderations<'_> {
    /// Classifies if text is potentially harmful.
    pub async fn create(
        &self,
        parameters: ModerationParameters,
    ) -> Result<ModerationResponse, APIError> {
        let response = self.client.post("/moderations", &parameters).await?;

        let moderation_response: ModerationResponse = format_response(response.data)?;

        Ok(moderation_response)
    }
}
