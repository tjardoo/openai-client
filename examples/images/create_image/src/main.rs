use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    BackgroundStyle, CreateImageParametersBuilder, ImageQuality, ImageSize, ModerationLevel,
    OutputFormat,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    // some parameters are commented out as they're only available for the ImageModel::DallE3
    let parameters = CreateImageParametersBuilder::default()
        .prompt("A cute dog in the park")
        .background(BackgroundStyle::Auto)
        .model(ImageModel::GptImage1.to_string())
        .moderation(ModerationLevel::Auto)
        .n(1u32)
        .output_compression(100u32)
        .output_format(OutputFormat::Png)
        .partial_images(0u32)
        .quality(ImageQuality::Auto)
        // .response_format(ResponseFormat::Url)
        .size(ImageSize::Size1024X1024)
        // .style(ImageStyle::Natural)
        .build()
        .unwrap();

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{paths:?}");

    println!("{result:#?}");
}
