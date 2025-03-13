use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::ToolModel;
use openai_dive::v1::resources::chat::{
    ApproximateUserLocation, ChatCompletionParametersBuilder, ChatCompletionResponseFormat,
    ChatMessage, ChatMessageContent, UserLocationType, WebSearchOptions, WebSearchUserLocation,
};
use openai_dive::v1::resources::shared::WebSearchContextSize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ToolModel::Gpt4OMiniSearchPreview.to_string())
        .messages(vec![ChatMessage::User {
            content: ChatMessageContent::Text(
                "What was a positive news story from today?!".to_string(),
            ),
            name: None,
        }])
        .web_search_options(WebSearchOptions {
            search_context_size: Some(WebSearchContextSize::Low),
            user_location: Some(ApproximateUserLocation {
                r#type: UserLocationType::Approximate,
                approximate: WebSearchUserLocation {
                    city: Some("Amsterdam".to_string()),
                    country: Some("NL".to_string()),
                    region: None,
                    timezone: None,
                },
            }),
        })
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
