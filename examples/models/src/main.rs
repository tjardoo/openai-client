use openai_dive::v1::{api::Client, models::Gpt4Model};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let result = client.models().list().await.unwrap();

    println!("{result:#?}");

    let result = client
        .models()
        .get(&Gpt4Model::Gpt4O.to_string())
        .await
        .unwrap();

    println!("{result:#?}");
}
