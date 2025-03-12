use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};
use openai_dive::v1::resources::response::shared::{ResponseTool, ResponseToolChoice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(Gpt4Engine::Gpt4OMini.to_string())
        .input(ResponseInput::Text(
            "What is the weather like in Ho Chi Minh City today?".to_string(),
        ))
        .tools(vec![ResponseTool::Function {
            name: "get_current_weather".to_string(),
            description: Some("Get the current weather in a given location.".to_string()),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    },
                },
                "additionalProperties": false,
                "required": ["location", "unit"]
            }),
            strict: false,
        }])
        .tool_choice(ResponseToolChoice::Auto)
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
