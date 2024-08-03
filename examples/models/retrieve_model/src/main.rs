use openai_dive::v1::{api::Client, models::Gpt4Engine};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client
        .models()
        .get(&Gpt4Engine::Gpt4O.to_string())
        .await
        .unwrap();

    println!("{:#?}", result);
}
