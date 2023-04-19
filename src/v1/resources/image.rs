use std::fmt::Display;
use serde::{Serialize, Deserialize};

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
    pub data: Vec<ImageUrl>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUrl {
    pub url: String,
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
