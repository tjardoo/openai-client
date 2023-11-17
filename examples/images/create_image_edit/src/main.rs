use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParameters, ImageSize};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditImageParameters {
        image: "./images/image_edit_original.png".to_string(),
        mask: Some("./images/image_edit_mask.png".to_string()),
        prompt: "A cute baby sea otter wearing a beret".to_string(),
        n: Some(1),
        size: Some(ImageSize::Size256X256),
        response_format: None,
        user: None,
    };

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:?}", result);
}
