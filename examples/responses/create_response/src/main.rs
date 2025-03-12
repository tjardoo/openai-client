use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .input(ResponseInput::Text("Hello, how are you?".to_string()))
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
