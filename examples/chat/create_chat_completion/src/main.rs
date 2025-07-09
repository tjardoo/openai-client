use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::ReasoningModel;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ReasoningModel::O3Mini.to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text("Hello!".to_string()),
                name: Some("Judy".to_string()),
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("What is the capital of Singapore?".to_string()),
                name: Some("Judy".to_string()),
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{result:#?}");

    for choice in &result.choices {
        if let Some(text) = choice.message.text() {
            println!("{text}");
        }
    }

    Ok(())
}
