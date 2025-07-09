use ftail::ansi_escape::TextStyling;
use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::endpoints::chat::RoleTrackingStream;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    DeltaChatMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY is not set");

    let mut client = Client::new(deepseek_api_key);
    client.set_base_url("https://api.deepseek.com");

    let parameters = ChatCompletionParametersBuilder::default()
        .model("deepseek-reasoner".to_string())
        .messages(vec![ChatMessage::User {
            content: ChatMessageContent::Text(
                "What are the five biggest cities in China?".to_string(),
            ),
            name: None,
        }])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

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
                            print!("{content}");
                        }
                        DeltaChatMessage::System { content, .. } => {
                            print!("{content}");
                        }
                        DeltaChatMessage::Assistant {
                            reasoning_content,
                            content,
                            ..
                        } => {
                            if let Some(reasoning_content) = reasoning_content {
                                print!("{}", reasoning_content.black());
                            }

                            if let Some(chat_message_content) = content {
                                print!("{chat_message_content}");
                            }
                        }
                        _ => {}
                    })
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}
