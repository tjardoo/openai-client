use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParametersBuilder, ChatCompletionTool,
    ChatCompletionToolType, ChatMessage, ChatMessageContent,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::Text(
            "Give me a random number higher than 100 but less than 2*150?".to_string(),
        ),
        name: None,
    }];

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(messages)
        .tools(vec![ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: "get_random_number".to_string(),
                description: Some("Get a random number between two values".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "min": {"type": "integer", "description": "Minimum value of the random number."},
                        "max": {"type": "integer", "description": "Maximum value of the random number."},
                    },
                    "required": ["min", "max"],
                }),
            },
        }])
        .build()
        .unwrap();

    let result = client.chat().create(parameters).await.unwrap();

    let message = result.choices[0].message.clone();

    if let ChatMessage::Assistant {
        content: _,
        refusal: _,
        name: _,
        tool_calls: Some(tool_calls),
    } = message
    {
        for tool_call in tool_calls {
            let name = tool_call.function.name;
            let arguments = tool_call.function.arguments;

            if name == "get_random_number" {
                let random_numbers: RandomNumber = serde_json::from_str(&arguments).unwrap();

                println!("Min: {:?}", &random_numbers.min);
                println!("Max: {:?}", &random_numbers.max);

                let random_number_result = get_random_number(random_numbers);

                println!(
                    "Random number between those numbers: {:?}",
                    random_number_result.clone()
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RandomNumber {
    min: u32,
    max: u32,
}

fn get_random_number(params: RandomNumber) -> Value {
    let random_number = rand::thread_rng().gen_range(params.min..params.max);

    random_number.into()
}
