use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = std::env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().retrieve(&file_id).await.unwrap();

    println!("{:#?}", result);
}
