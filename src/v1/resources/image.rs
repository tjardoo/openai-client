use std::fmt::Display;
use serde::{Serialize, Deserialize};
use crate::v1::error::APIError;

#[cfg(feature = "download")]
use futures::future;

#[derive(Serialize, Debug)]
pub struct CreateImageParameters {
    pub prompt: String,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub number_of_images: Option<u32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Debug)]
pub struct EditImageParameters {
    pub image: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub number_of_images: Option<u32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Debug)]
pub struct CreateImageVariationParameters {
    pub image: String,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub number_of_images: Option<u32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Size256X256,
    #[serde(rename = "512x512")]
    Size512X512,
    #[serde(rename = "1024x1024")]
    Size1024X1024,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResponse {
    pub created: u32,
    pub data: Vec<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageData {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "b64_json")]
    B64Json(String),
}

impl ImageResponse {
    #[cfg(feature = "download")]
    pub async fn save(&self, path: &str) -> Result<Vec<String>, APIError> {
        let mut files = vec![];
        let mut handles = vec![];

        for item in self.data.clone() {
            let path = path.to_owned();

            handles.push(tokio::spawn(async move {
                item.save_to_disk(&path).await
            }));
        }

        let results = future::join_all(handles).await;

        for result in results {
            match result {
                Ok(path) => match path {
                    Ok(item) => files.push(item),
                    Err(_error) => (),
                }
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
            ImageData::Url(url) => self.download_image_from_url(url, path).await,
            ImageData::B64Json(b64_json) => self.download_b64_json_image(b64_json, path).await,
        }
    }

    #[cfg(feature = "download")]
    async fn download_image_from_url(&self, url: &str, path: &str) -> Result<String, APIError> {
        use super::shared::generate_file_name;

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
        ).await
        .map_err(|error| APIError::FileError(error.to_string()))?;

        Ok(full_path)
    }

    #[cfg(feature = "download")]
    async fn download_b64_json_image(&self, b64_json: &str, _path: &str) -> Result<String, APIError> {
        let _response = b64_json;

        Ok("todo path".to_string())
    }
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                ImageSize::Size256X256 => "256x256",
                ImageSize::Size512X512 => "512x512",
                ImageSize::Size1024X1024 => "1024x1024",
            }
        )
    }
}

impl Display for ResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                ResponseFormat::Url => "url",
                ResponseFormat::B64Json => "b64_json",
            }
        )
    }
}
