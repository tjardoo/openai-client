use openai_dive::v1::api::Client;
use openai_dive::v1::models::DallEEngine;
use openai_dive::v1::resources::image::{
    CreateImageParameters, ImageQuality, ImageSize, ImageStyle, ResponseFormat,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageParameters {
        prompt: "A cute dog in the park".to_string(),
        model: Some(DallEEngine::DallE3.to_string()),
        n: Some(1),
        quality: Some(ImageQuality::Standard),
        response_format: Some(ResponseFormat::Url),
        size: Some(ImageSize::Size1024X1024),
        style: Some(ImageStyle::Natural),
        user: None,
    };

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{:?}", paths);

    println!("{:#?}", result);
}
