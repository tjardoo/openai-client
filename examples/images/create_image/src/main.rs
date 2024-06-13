use openai_dive::v1::api::Client;
use openai_dive::v1::models::DallEEngine;
use openai_dive::v1::resources::image::{
    CreateImageParametersBuilder, ImageQuality, ImageSize, ImageStyle, ResponseFormat,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageParametersBuilder::default()
        .prompt("A cute dog in the park")
        .model(DallEEngine::DallE3.to_string())
        .n(1u32)
        .quality(ImageQuality::Standard)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::Size1024X1024)
        .style(ImageStyle::Natural)
        .build()
        .unwrap();

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{:?}", paths);

    println!("{:#?}", result);
}
