use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let file_id = std::env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().delete(&file_id).await.unwrap();

    println!("{:#?}", result);
}
