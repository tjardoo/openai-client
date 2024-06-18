#[cfg(feature = "download")]
use crate::v1::error::APIError;
#[cfg(feature = "download")]
use crate::v1::helpers::generate_file_name;
#[cfg(feature = "download")]
use base64::{engine::general_purpose, Engine as _};
use derive_builder::Builder;
#[cfg(feature = "download")]
use futures::future;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumString;

#[derive(Serialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateImageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateImageParameters {
    /// A text description of the desired image(s). The maximum length is 1000 characters for dall-e-2 and 4000 characters for dall-e-3.
    pub prompt: String,
    /// The model to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The number of images to generate. Must be between 1 and 10. For dall-e-3, only n=1 is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// The quality of the image that will be generated. hd creates images with finer details and greater consistency across the image.
    /// This param is only supported for dall-e-3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// The format in which the generated images are returned. Must be one of url or b64_json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// The size of the generated images. Must be one of 256x256, 512x512, or 1024x1024 for dall-e-2.
    /// Must be one of 1024x1024, 1792x1024, or 1024x1792 for dall-e-3 models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
    /// The style of the generated images. Must be one of vivid or natural.
    /// Vivid causes the model to lean towards generating hyper-real and dramatic images.
    /// Natural causes the model to produce more natural, less hyper-real looking images.
    /// This param is only supported for dall-e-3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "EditImageParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct EditImageParameters {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square. If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: String,
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,
    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where image should be edited.
    /// Must be a valid PNG file, less than 4MB, and have the same dimensions as image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    /// The model to use for image generation. Only dall-e-2 is supported at this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The number of images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// The size of the generated images. Must be one of 256x256, 512x512, or 1024x1024.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
    /// The format in which the generated images are returned. Must be one of url or b64_json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateImageVariationParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateImageVariationParameters {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
    pub image: String,
    /// The model to use for image generation. Only dall-e-2 is supported at this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The number of images to generate. Must be between 1 and 10. For dall-e-3, only n=1 is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// The format in which the generated images are returned. Must be one of url or b64_json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// The size of the generated images. Must be one of 256x256, 512x512, or 1024x1024.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageResponse {
    /// The Unix timestamp (in seconds) for when the image was created.
    pub created: u32,
    /// The base64-encoded JSON of the generated image, if response_format is b64_json.
    /// The URL of the generated image, if response_format is url (default).
    pub data: Vec<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Size256X256,
    #[serde(rename = "512x512")]
    Size512X512,
    #[serde(rename = "1024x1024")]
    Size1024X1024,
    #[serde(rename = "1792x1024")]
    Size1792X1024,
    #[serde(rename = "1024x1792")]
    Size1024X1792,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    Standard,
    Hd,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    Vivid,
    Natural,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Url,
    B64Json,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, PartialEq)]
#[serde(untagged)]
pub enum ImageData {
    Url {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        revised_prompt: Option<String>,
    },
    B64Json {
        b64_json: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        revised_prompt: Option<String>,
    },
}

impl ImageResponse {
    #[cfg(feature = "download")]
    pub async fn save(&self, path: &str) -> Result<Vec<String>, APIError> {
        let mut files = vec![];
        let mut handles = vec![];

        for item in self.data.clone() {
            let path = path.to_owned();

            handles.push(tokio::spawn(async move { item.save_to_disk(&path).await }));
        }

        let results = future::join_all(handles).await;

        for result in results {
            match result {
                Ok(path) => match path {
                    Ok(item) => files.push(item),
                    Err(_error) => (),
                },
                Err(_error) => (),
            }
        }

        Ok(files)
    }
}

impl ImageData {
    #[cfg(feature = "download")]
    pub async fn save_to_disk(&self, path: &str) -> Result<String, APIError> {
        match self {
            ImageData::Url { url, .. } => self.download_image_from_url(url, path).await,
            ImageData::B64Json { b64_json, .. } => {
                self.download_b64_json_image(b64_json, path).await
            }
        }
    }

    #[cfg(feature = "download")]
    async fn download_image_from_url(&self, url: &str, path: &str) -> Result<String, APIError> {
        let response = reqwest::get(url)
            .await
            .map_err(|error| APIError::FileError(error.to_string()))?;

        let full_path = generate_file_name(path, 12, "png");

        tokio::fs::write(
            &full_path,
            response
                .bytes()
                .await
                .map_err(|error| APIError::FileError(error.to_string()))?,
        )
        .await
        .map_err(|error| APIError::FileError(error.to_string()))?;

        Ok(full_path)
    }

    #[cfg(feature = "download")]
    async fn download_b64_json_image(
        &self,
        b64_json: &str,
        path: &str,
    ) -> Result<String, APIError> {
        let full_path = generate_file_name(path, 12, "png");

        let bytes = general_purpose::STANDARD.decode(b64_json).unwrap();

        tokio::fs::write(&full_path, bytes)
            .await
            .map_err(|error| APIError::FileError(error.to_string()))?;

        Ok(full_path)
    }
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageSize::Size256X256 => "256x256",
                ImageSize::Size512X512 => "512x512",
                ImageSize::Size1024X1024 => "1024x1024",
                ImageSize::Size1792X1024 => "1792x1024",
                ImageSize::Size1024X1792 => "1024x1792",
            }
        )
    }
}

impl Display for ResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResponseFormat::Url => "url",
                ResponseFormat::B64Json => "b64_json",
            }
        )
    }
}
