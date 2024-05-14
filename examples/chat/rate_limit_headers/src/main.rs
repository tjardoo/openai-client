use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParameters, ChatCompletionResponse, ChatMessage, ChatMessageContent, Role,
};
use openai_dive::v1::resources::shared::ResponseWrapper;
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
                content: ChatMessageContent::Text(
                    "Which country has the largest population?".to_string(),
                ),
                ..Default::default()
            },
        ],
        max_tokens: Some(50),
        ..Default::default()
    };

    let result: ResponseWrapper<ChatCompletionResponse> =
        client.chat().create_wrapped(parameters).await.unwrap();

    println!("{:#?}", result.headers);

    println!("{:#?}", result.data);
}
