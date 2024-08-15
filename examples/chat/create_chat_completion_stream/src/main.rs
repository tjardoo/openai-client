use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::endpoints::chat::RoleTrackingStream;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    DeltaChatMessage,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text("Hello!".to_string()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    "What are the five biggest cities in Vietnam?".to_string(),
                ),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()
        .unwrap();

    let stream = client.chat().create_stream(parameters).await.unwrap();

    let mut tracked_stream = RoleTrackingStream::new(stream);

    while let Some(response) = tracked_stream.next().await {
        match response {
            Ok(chat_response) => {
                chat_response
                    .choices
                    .iter()
                    .for_each(|choice| match &choice.delta {
                        DeltaChatMessage::User { content, .. } => {
                            print!("{}", content);
                        }
                        DeltaChatMessage::System { content, .. } => {
                            print!("{}", content);
                        }
                        DeltaChatMessage::Assistant {
                            content: Some(chat_message_content),
                            ..
                        } => {
                            print!("{}", chat_message_content);
                        }
                        _ => {}
                    })
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
