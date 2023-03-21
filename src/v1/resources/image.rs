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
    pub data: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
}
