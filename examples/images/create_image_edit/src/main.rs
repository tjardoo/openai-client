use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParametersBuilder, ImageSize};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditImageParametersBuilder::default()
        .image(FileUpload::File(
            "./images/image_edit_original.png".to_string(),
        ))
        .prompt("A cute baby sea otter")
        .mask(FileUpload::File("./images/image_edit_mask.png".to_string()))
        .n(1u32)
        .size(ImageSize::Size512X512)
        .build()
        .unwrap();

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:#?}", result);
}
