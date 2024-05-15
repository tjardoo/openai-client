use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let batch_id = env::var("BATCH_ID").expect("BATCH_ID is not set in the .env file.");

    let result = client.batches().retrieve(&batch_id).await.unwrap();

    println!("{:#?}", result);
}
