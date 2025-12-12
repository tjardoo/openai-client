use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    BackgroundStyle, EditImageParametersBuilder, ImageQuality, ImageSize, MimeType, OutputFormat,
    ResponseFormat,
};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = EditImageParametersBuilder::default()
        .image(FileUpload::File(
            "./images/image_edit_original.png".to_string(),
        ))
        .prompt("A cute baby sea otter")
        .background(BackgroundStyle::Auto)
        .mask(FileUpload::File("./images/image_edit_mask.png".to_string()))
        .model(ImageModel::DallE3.to_string())
        .n(1u32)
        .mime_type(MimeType::Png)
        .output_compression(100u32)
        .output_format(OutputFormat::Png)
        .partial_images(0u32)
        .quality(ImageQuality::Auto)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::Size1024X1024)
        .build()
        .unwrap();

    let result = client.images().edit(parameters).await.unwrap();

    println!("{result:#?}");
}
