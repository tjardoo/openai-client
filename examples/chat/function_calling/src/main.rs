use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParameters, ChatCompletionTool, ChatCompletionToolChoice,
    ChatCompletionToolChoiceFunction, ChatCompletionToolChoiceFunctionName, ChatCompletionToolType,
    ChatMessage, Role,
};
use openai_dive::v1::resources::shared::FinishReason;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let mut messages = vec![ChatMessage {
        content: Some("Give me a random number between 25 and 50?".to_string()),
        ..Default::default()
    }];

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages: messages.clone(),
        tool_choice: Some(ChatCompletionToolChoice::ChatCompletionToolChoiceFunction(
            ChatCompletionToolChoiceFunction {
                r#type: Some(ChatCompletionToolType::Function),
                function: ChatCompletionToolChoiceFunctionName {
                    name: "get_random_number".to_string(),
                },
            },
        )),
        tools: Some(vec![ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: "get_random_number".to_string(),
                description: Some("Get a random number between two values".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "min": {"type": "integer", "description": "Minimum value of the random number."},
                        "max": {"type": "integer", "description": "Maximum value of the random number."},
                    }
                }),
            },
        }]),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    for choice in result.choices.iter() {
        if choice.finish_reason == FinishReason::StopSequenceReached {
            if let Some(tool_calls) = &choice.message.tool_calls {
                for tool_call in tool_calls.iter() {
                    let random_numbers =
                        serde_json::from_str(&tool_call.function.arguments).unwrap();

                    if tool_call.function.name == "get_random_number" {
                        let random_number_result = get_random_number(random_numbers);

                        messages.push(ChatMessage {
                            role: Role::Function,
                            content: Some(serde_json::to_string(&random_number_result).unwrap()),
                            name: Some("get_random_number".to_string()),
                            ..Default::default()
                        });

                        let parameters = ChatCompletionParameters {
                            model: "gpt-3.5-turbo-0613".to_string(),
                            messages: messages.clone(),
                            ..Default::default()
                        };

                        let result = client.chat().create(parameters).await.unwrap();

                        println!("{:#?}", result);
                    }
                }
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
