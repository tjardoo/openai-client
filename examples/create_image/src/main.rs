use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageParameters, ImageSize, ResponseFormat};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageParameters {
        prompt: "A cute baby dog".to_string(),
        number_of_images: Some(2),
        image_size: Some(ImageSize::Size256X256),
        response_format: Some(ResponseFormat::Url),
        // response_format: Some(ResponseFormat::B64Json),
    };

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{:?}", paths);

    println!("{:?}", result);
}
