use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{SimpleChatCompletionParameters, ChatMessage, Role};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = SimpleChatCompletionParameters {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                name: None,
            },
            ChatMessage {
                role: Role::User,
                content: "Where are you located?".to_string(),
                name: None,
            },
        ],
        max_tokens: 12,
    };

    let result = client.chat().create_simple(parameters).await.unwrap();

    println!("{:?}", result);
}
