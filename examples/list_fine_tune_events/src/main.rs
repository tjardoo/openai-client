use std::env;
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.fine_tunes().list_events("ft-eJ3GlifDi5MEgkLtg6zACNFl").await.unwrap();

    println!("{:?}", result);
}
