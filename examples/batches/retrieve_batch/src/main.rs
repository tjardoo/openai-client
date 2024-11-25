use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let batch_id = std::env::var("BATCH_ID").expect("BATCH_ID is not set in the .env file.");

    let result = client.batches().retrieve(&batch_id).await.unwrap();

    println!("{:#?}", result);
}
