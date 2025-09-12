use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::image::{
    CreateImageParameters, CreateImageVariationParameters, EditImageParameters, ImageResponse,
};
use crate::v1::resources::shared::FileUpload;

pub struct Images<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Given a prompt and/or an input image, the model will generate a new image.
    pub fn images(&self) -> Images<'_> {
        Images { client: self }
    }
}

impl Images<'_> {
    /// Creates an image given a prompt.
    pub async fn create(
        &self,
        parameters: CreateImageParameters,
    ) -> Result<ImageResponse, APIError> {
        let response = self
            .client
            .post("/images/generations", &parameters, None)
            .await?;

        let response: ImageResponse = format_response(response.data)?;

        Ok(response)
    }

    /// Creates an edited or extended image given an original image and a prompt.
    pub async fn edit(&self, parameters: EditImageParameters) -> Result<ImageResponse, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let mime_type = parameters.mime_type;

        match parameters.image {
            #[cfg(all(feature = "tokio", feature = "tokio-util"))]
            FileUpload::File(_) => {
                let mut image = parameters.image.into_part().await?;

                if let Some(ref mime_type) = mime_type {
                    image = image
                        .mime_str(&mime_type.to_string())
                        .map_err(|error| APIError::FileError(error.to_string()))?;
                }
                form = form.part("image", image);
            }
            #[cfg(all(feature = "tokio", feature = "tokio-util"))]
            FileUpload::FileArray(_) => {
                let images = parameters.image.into_parts().await?;
                for mut image in images {
                    if let Some(ref mime_type) = mime_type {
                        image = image
                            .mime_str(&mime_type.to_string())
                            .map_err(|error| APIError::FileError(error.to_string()))?;
                    }
                    form = form.part("image[]", image);
                }
            }
            FileUpload::Bytes(_) => {
                let mut image = parameters.image.into_part().await?;

                if let Some(ref mime_type) = mime_type {
                    image = image
                        .mime_str(&mime_type.to_string())
                        .map_err(|error| APIError::FileError(error.to_string()))?;
                }
                form = form.part("image", image);
            }
            FileUpload::BytesArray(_) => {
                let images = parameters.image.into_parts().await?;
                for mut image in images {
                    if let Some(ref mime_type) = mime_type {
                        image = image
                            .mime_str(&mime_type.to_string())
                            .map_err(|error| APIError::FileError(error.to_string()))?;
                    }
                    form = form.part("image[]", image);
                }
            }
        }

        form = form.text("prompt", parameters.prompt);

        if let Some(background) = parameters.background {
            form = form.text("background", background.to_string());
        }

        if let Some(quality) = parameters.quality {
            form = form.text("quality", quality.to_string());
        }

        if let Some(mask) = parameters.mask {
            let image = mask.into_part().await?;
            form = form.part("mask", image);
        }

        if let Some(model) = parameters.model {
            form = form.text("model", model);
        }

        if let Some(n) = parameters.n {
            form = form.text("n", n.to_string());
        }

        if let Some(size) = parameters.size {
            form = form.text("size", size.to_string());
        }

        if let Some(response_format) = parameters.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(user) = parameters.user {
            form = form.text("user", user.to_string());
        }

        let response = self.client.post_with_form("/images/edits", form).await?;

        let response: ImageResponse = format_response(response)?;

        Ok(response)
    }

    /// Creates a variation of a given image.
    pub async fn variation(
        &self,
        parameters: CreateImageVariationParameters,
    ) -> Result<ImageResponse, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let image = parameters.image.into_part().await?;
        form = form.part("image", image);

        if let Some(model) = parameters.model {
            form = form.text("model", model);
        }

        if let Some(n) = parameters.n {
            form = form.text("n", n.to_string());
        }

        if let Some(response_format) = parameters.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(size) = parameters.size {
            form = form.text("size", size.to_string());
        }

        if let Some(user) = parameters.user {
            form = form.text("user", user.to_string());
        }

        let response = self
            .client
            .post_with_form("/images/variations", form)
            .await?;

        let response: ImageResponse = format_response(response)?;

        Ok(response)
    }
}
