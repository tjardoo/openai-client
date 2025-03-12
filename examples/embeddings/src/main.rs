use openai_dive::v1::api::Client;
use openai_dive::v1::models::EmbeddingsEngine;
use openai_dive::v1::resources::embedding::{
    EmbeddingEncodingFormat, EmbeddingInput, EmbeddingParametersBuilder,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = EmbeddingParametersBuilder::default()
        .model(EmbeddingsEngine::TextEmbedding3Small.to_string())
        .input(EmbeddingInput::String(
            "The food was delicious and the waiter...".to_string(),
        ))
        .encoding_format(EmbeddingEncodingFormat::Float)
        .build()
        .unwrap();

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
