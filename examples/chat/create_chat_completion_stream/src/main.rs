use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParameters, ChatMessage, ChatMessageContent, Role,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt4O.to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("Hello!".to_string()),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("What is the capital of Vietnam?".to_string()),
                ..Default::default()
            },
        ],
        max_tokens: Some(12),
        ..Default::default()
    };

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
