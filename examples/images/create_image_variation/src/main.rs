use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    CreateImageVariationParametersBuilder, ImageSize, ResponseFormat,
};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = CreateImageVariationParametersBuilder::default()
        .image(FileUpload::File(
            "./images/image_edit_original.png".to_string(),
        ))
        .model(ImageModel::DallE3.to_string())
        .n(1u32)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::Size1024X1024)
        .build()
        .unwrap();

    let result = client.images().variation(parameters).await.unwrap();

    println!("{result:#?}");
}
