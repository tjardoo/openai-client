use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().retrieve_content(&file_id).await.unwrap();

    println!("{:?}", result);
}
