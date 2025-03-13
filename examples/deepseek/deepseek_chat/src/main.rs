use openai_dive::v1::{
    api::Client,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY is not set");

    let mut client = Client::new(deepseek_api_key);
    client.set_base_url("https://api.deepseek.com");

    let result = client.models().list().await.unwrap();

    println!("{:#?}", result);

    let parameters = ChatCompletionParametersBuilder::default()
        .model("deepseek-chat".to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text("I want to learn Mandarin. How can I best approach this? I already used the DuoLingo app for a half a year.".to_string()),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
