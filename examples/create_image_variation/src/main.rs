use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageVariationParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageVariationParameters {
        image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/tree/master/OpenAI.Playground/SampleData
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let result = client.images().variation(parameters).await.unwrap();

    println!("{:?}", result);
}
