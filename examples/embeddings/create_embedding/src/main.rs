use openai_dive::v1::api::Client;
use openai_dive::v1::models::EmbeddingsEngine;
use openai_dive::v1::resources::embedding::{
    EmbeddingEncodingFormat, EmbeddingInput, EmbeddingParameters,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParameters {
        model: EmbeddingsEngine::TextEmbedding3Small.to_string(),
        input: EmbeddingInput::String("The food was delicious and the waiter...".to_string()),
        encoding_format: Some(EmbeddingEncodingFormat::Float),
        dimensions: None,
        user: None,
    };

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
