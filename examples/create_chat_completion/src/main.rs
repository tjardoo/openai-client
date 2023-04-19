use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        max_tokens: 12,
        temperature: None,
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:?}", result);
}
