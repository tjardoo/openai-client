use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};
use openai_dive::v1::resources::response::response::ResponseReasoning;
use openai_dive::v1::resources::shared::ReasoningEffort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model("o1".to_string())
        .input(ResponseInput::Text(
            "How much wood would a woodchuck chuck?".to_string(),
        ))
        .reasoning(ResponseReasoning {
            effort: Some(ReasoningEffort::Low),
            // generate_summary: Some(ReasoningSummary::Concise),
        })
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
