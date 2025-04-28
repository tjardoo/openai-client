use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::CostOptimizedModel;
use openai_dive::v1::resources::response::items::{FunctionToolCallOutput, InputItemStatus};
use openai_dive::v1::resources::response::request::{InputItem, ResponseInput, ResponseInputItem, ResponseParametersBuilder};
use openai_dive::v1::resources::response::response::ResponseOutput;
use openai_dive::v1::resources::response::shared::{ResponseTool, ResponseToolChoice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = ResponseParametersBuilder::default()
        .model(CostOptimizedModel::Gpt4OMini.to_string())
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

    let call = match &result.output[0] {
        ResponseOutput::FunctionToolCall(call) => call,
        _ => panic!("unexpected output"),
    };

    let parameters = ResponseParametersBuilder::default()
        .model(CostOptimizedModel::Gpt4OMini.to_string())
        .input(ResponseInput::List(vec![ResponseInputItem::Item(InputItem::FunctionToolCallOutput(FunctionToolCallOutput {
            id: None,
            call_id: call.call_id.clone(),
            output: "{\"temperature_2m\":30,\"wind_speed_10m\":5}".to_string(),
            status: InputItemStatus::Completed,
        }))]))
        .previous_response_id(result.id)
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
