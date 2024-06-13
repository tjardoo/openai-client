use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParametersBuilder, ImageSize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditImageParametersBuilder::default()
        .image("./images/image_edit_original.png")
        .prompt("A cute baby sea otter")
        .mask("./images/image_edit_mask.png")
        .n(1u32)
        .size(ImageSize::Size512X512)
        .build()?;

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:#?}", result);

    Ok(())
}
