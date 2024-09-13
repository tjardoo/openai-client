use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // used to log the JSON response from the OpenAI API
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

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
                content: ChatMessageContent::Text("What is the capital of Vietnam?".to_string()),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
