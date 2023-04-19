use std::env;
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.fine_tunes().delete_fine_tune_model("curie:ft-personal-XXX").await.unwrap();

    println!("{:?}", result);
}
