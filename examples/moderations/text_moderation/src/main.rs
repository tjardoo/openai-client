use openai_dive::v1::api::Client;
use openai_dive::v1::models::ModerationsEngine;
use openai_dive::v1::resources::moderation::{ModerationInput, ModerationParametersBuilder};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = ModerationParametersBuilder::default()
        .model(ModerationsEngine::OmniModerationLatest.to_string())
        // .input(ModerationInput::Text("I want to kill them.".to_string()))
        .input(ModerationInput::Array(vec![
            "I want to kill them.".to_string()
        ]))
        .build()
        .unwrap();

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
