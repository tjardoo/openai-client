use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParametersBuilder, ImageSize, MimeType};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = EditImageParametersBuilder::default()
        .image(FileUpload::File(
            "./images/image_edit_original.png".to_string(),
        ))
        .prompt("A cute baby sea otter")
        .mask(FileUpload::File("./images/image_edit_mask.png".to_string()))
        .n(1u32)
        .mime_type(MimeType::Png)
        .size(ImageSize::Size512X512)
        .build()
        .unwrap();

    let result = client.images().edit(parameters).await.unwrap();

    println!("{result:#?}");
}
