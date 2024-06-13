use openai_dive::v1::api::Client;
use openai_dive::v1::models::EmbeddingsEngine;
use openai_dive::v1::resources::embedding::{
    EmbeddingEncodingFormat, EmbeddingInput, EmbeddingParametersBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParametersBuilder::default()
        .model(EmbeddingsEngine::TextEmbedding3Small.to_string())
        .input(EmbeddingInput::String(
            "The food was delicious and the waiter...".to_string(),
        ))
        .encoding_format(EmbeddingEncodingFormat::Float)
        .build()?;

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:#?}", result);

    Ok(())
}
