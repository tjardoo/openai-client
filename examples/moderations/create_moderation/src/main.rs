use openai_dive::v1::api::Client;
use openai_dive::v1::models::ModerationsEngine;
use openai_dive::v1::resources::moderation::ModerationParametersBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ModerationParametersBuilder::default()
        .model(ModerationsEngine::TextModerationLatest.to_string())
        .input("I want to kill them.".to_string())
        .build()?;

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:#?}", result);

    Ok(())
}
