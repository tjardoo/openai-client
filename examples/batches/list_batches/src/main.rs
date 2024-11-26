use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let result = client.batches().list(None).await.unwrap();

    println!("{:#?}", result);
}
