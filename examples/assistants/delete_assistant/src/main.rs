use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let result = client.assistants().delete(&assistant_id).await.unwrap();

    println!("{:?}", result);
}
