use std::env;
use futures::StreamExt;
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

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if choice.delta.role.is_some() {
                    println!("role: {}", choice.delta.role.as_ref().unwrap());
                } else if choice.delta.content.is_some() {
                    print!("{}", choice.delta.content.as_ref().unwrap());
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
