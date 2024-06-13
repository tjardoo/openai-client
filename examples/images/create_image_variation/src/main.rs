use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageVariationParametersBuilder, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageVariationParametersBuilder::default()
        .image("./images/image_edit_original.png")
        .n(1u32)
        .size(ImageSize::Size256X256)
        .build()
        .unwrap();

    let result = client.images().variation(parameters).await.unwrap();

    println!("{:#?}", result);
}
