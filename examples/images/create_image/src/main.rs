use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    BackgroundStyle, CreateImageParametersBuilder, ImageQuality, ImageSize, ImageStyle,
    ModerationLevel, OutputFormat, ResponseFormat,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = CreateImageParametersBuilder::default()
        .prompt("A cute dog in the park")
        .background(BackgroundStyle::Auto)
        .model(ImageModel::DallE3.to_string())
        .moderation(ModerationLevel::Auto)
        .n(1u32)
        .output_compression(100u32)
        .output_format(OutputFormat::Png)
        .partial_images(0u32)
        .quality(ImageQuality::Auto)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::Size1024X1024)
        .style(ImageStyle::Natural)
        .build()
        .unwrap();

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{paths:?}");

    println!("{result:#?}");
}
