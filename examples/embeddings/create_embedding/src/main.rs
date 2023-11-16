use openai_dive::v1::api::Client;
use openai_dive::v1::resources::embedding::EmbeddingParameters;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParameters {
        model: "text-embedding-ada-002".to_string(),
        input: "The food was delicious and the waiter...".to_string(),
        encoding_format: None,
        user: None,
    };

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:?}", result);
}
