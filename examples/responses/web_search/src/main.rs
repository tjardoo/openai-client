use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::FlagshipModel;
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};
use openai_dive::v1::resources::response::shared::{
    ResponseTool, ResponseToolChoice, UserLocationType, WebSearchUserLocation,
};
use openai_dive::v1::resources::shared::WebSearchContextSize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(FlagshipModel::Gpt4O.to_string())
        .input(ResponseInput::Text(
            "What was a positive news story from today?".to_string(),
        ))
        .tools(vec![ResponseTool::WebSearch {
            search_context_size: Some(WebSearchContextSize::Low),
            user_location: Some(WebSearchUserLocation {
                r#type: UserLocationType::Approximate,
                city: Some("Bangkok".to_string()),
                country: Some("TH".to_string()),
                region: None,
                timezone: None,
            }),
        }])
        .tool_choice(ResponseToolChoice::WebSearchPreview)
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{result:#?}");

    Ok(())
}
