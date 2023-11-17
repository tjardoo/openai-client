use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_name = "file-GXWaNDmW2ekjs81jY8MuWPYm";

    let result = client.files().delete(file_name).await.unwrap();

    println!("{:?}", result);
}
