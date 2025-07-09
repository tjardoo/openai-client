use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    CreateImageParametersBuilder, ImageQuality, ImageSize, ImageStyle, ResponseFormat,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = CreateImageParametersBuilder::default()
        .prompt("A cute dog in the park")
        .model(ImageModel::DallE3.to_string())
        .n(1u32)
        .quality(ImageQuality::Standard)
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
