use openai_dive::v1::api::Client;
use openai_dive::v1::models::ModerationsEngine;
use openai_dive::v1::resources::moderation::ModerationParameters;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ModerationParameters {
        input: "I want to kill them.".to_string(),
        model: ModerationsEngine::TextModerationLatest.to_string(),
    };

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
