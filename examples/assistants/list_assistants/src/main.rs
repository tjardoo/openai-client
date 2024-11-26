use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let result = client.assistants().list(None).await.unwrap();

    println!("{:#?}", result);
}
