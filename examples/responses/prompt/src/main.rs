use std::collections::HashMap;

use openai_dive::v1::api::Client;
use openai_dive::v1::models::FlagshipModel;
use openai_dive::v1::resources::response::request::{
    Prompt, ResponseInput, ResponseParametersBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(FlagshipModel::Gpt41.to_string())
        .prompt(Prompt {
            id: "pmpt_687f87cf5ef08193aa802807cdec714a0759a65210f9a772".to_string(),
            version: None,
            variables: Some(HashMap::from([(
                "country".to_string(),
                "Switzerland".to_string(),
            )])),
        })
        .input(ResponseInput::Text(
            "Who's the president of the European Union?".to_string(),
        ))
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{result:#?}");

    Ok(())
}
