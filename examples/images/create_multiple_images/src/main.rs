use openai_dive::v1::api::Client;
use openai_dive::v1::models::ImageModel;
use openai_dive::v1::resources::image::{
    EditImageParametersBuilder, ImageQuality, ImageSize, MimeType,
};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = EditImageParametersBuilder::default()
        .prompt("Make this person smile with full teeth")
        .image(FileUpload::File("./images/person.png".to_string()))
        .model(ImageModel::GptImage1.to_string())
        .quality(ImageQuality::Low)
        .mime_type(MimeType::Png)
        .n(1u32)
        .size(ImageSize::Size1024X1024)
        .build()?;

    let result = client.images().edit(parameters).await?;

    println!("{result:#?}");

    let parameters = EditImageParametersBuilder::default()
        .prompt("Combine the person into the orginal image")
        .image(FileUpload::FileArray(vec![
            "./images/image_edit_original.png".to_string(),
            "./images/person.png".to_string(),
        ]))
        .model("gpt-image-1")
        .quality(ImageQuality::Low)
        .mime_type(MimeType::Png)
        .n(1u32)
        .size(ImageSize::Size1024X1024)
        .build()?;

    let result = client.images().edit(parameters).await?;

    println!("{result:#?}");

    Ok(())
}
