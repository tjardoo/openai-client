use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
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
        temperature: None,
        top_p: None,
        n: None,
        stop: None,
        max_tokens: Some(12),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:?}", result);
}
