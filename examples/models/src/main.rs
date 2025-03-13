use openai_dive::v1::{api::Client, models::FlagshipModel};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let result = client.models().list().await.unwrap();

    println!("{:#?}", result);

    let result = client
        .models()
        .get(&FlagshipModel::Gpt4O.to_string())
        .await
        .unwrap();

    println!("{:#?}", result);
}
