use futures::stream::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Model;
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};
use openai_dive::v1::resources::response::response::ResponseStreamEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(Gpt4Model::Gpt41.to_string())
        .input(ResponseInput::Text(
            "Who's the president of the European Union?".to_string(),
        ))
        .build()?;

    let mut stream = client.responses().create_stream(parameters).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response_event) => match &response_event {
                ResponseStreamEvent::ResponseOutputTextDelta { delta, .. } => {
                    print!("{}", delta);
                }
                ResponseStreamEvent::ResponseCompleted { .. }
                | ResponseStreamEvent::ResponseIncomplete { .. }
                | ResponseStreamEvent::ResponseFailed { .. } => {
                    break;
                }
                _ => {
                    println!("{response_event:#?}");
                }
            },
            Err(e) => {
                eprintln!("{e:#?}");
            }
        }
    }

    Ok(())
}
