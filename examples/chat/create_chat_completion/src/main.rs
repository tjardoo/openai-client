use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat,
    ChatCompletionResponseFormatType, ChatMessageBuilder, ChatMessageContent, Role,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text("Hello!".to_string()))
                .build()
                .unwrap(),
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text(
                    "What is the capital of Vietnam?".to_string(),
                ))
                .build()
                .unwrap(),
        ])
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::Text,
        })
        .build()
        .unwrap();

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
