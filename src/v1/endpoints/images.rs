use crate::v1::resources::image::{CreateImageParameters, ImageResponse};
use crate::v1::{api::Client, error::APIError};
use serde_json::Value;

pub struct Images<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn images(&self) -> Images {
        Images {
            client: self,
        }
    }
}

impl Images<'_> {
    pub async fn create(&self, parameters: CreateImageParameters) -> Result<ImageResponse, APIError> {
        let response = self.client.post("/images/generations", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let create_image_response: ImageResponse = serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(create_image_response)
    }
}
