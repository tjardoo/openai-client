use openai_dive::v1::{
    api::Client,
    resources::file::{FilePurpose, ListFilesParameters},
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let query = ListFilesParameters {
        purpose: Some(FilePurpose::FineTune),
    };

    let result = client.files().list(Some(query)).await.unwrap();

    println!("{:?}", result);
}
