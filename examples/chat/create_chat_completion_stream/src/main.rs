use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat,
    ChatCompletionResponseFormatType, ChatMessageBuilder, ChatMessageContent, Role,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text("Hello!".to_string()))
                .build()?,
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text(
                    "What is the capital of Vietnam?".to_string(),
                ))
                .build()?,
        ])
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::Text,
        })
        .build()?;

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

    Ok(())
}
