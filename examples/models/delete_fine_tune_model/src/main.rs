use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().delete("my-custom-model").await.unwrap();

    println!("{:?}", result);
}
