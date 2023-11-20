use futures::executor::block_on;
use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParameters, ChatCompletionTool, ChatCompletionToolChoice,
    ChatCompletionToolChoiceFunction, ChatCompletionToolChoiceFunctionName, ChatCompletionToolType,
    ChatMessage, DeltaFunction, Role,
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

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    let mut function = DeltaFunction {
        name: None,
        arguments: None,
    };

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_chunk_response) => chat_chunk_response.choices.iter().for_each(|choice| {
                if let Some(delta_tool_calls) = &choice.delta.tool_calls {
                    function.merge(&delta_tool_calls.first().unwrap().function.clone());
                } else if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }

                if choice.finish_reason == Some(FinishReason::StopSequenceReached) {
                    let random_numbers =
                        serde_json::from_str(function.clone().arguments.unwrap().as_ref()).unwrap();

                    if let Some(name) = function.clone().name {
                        if name == "get_random_number" {
                            let random_number_result = get_random_number(random_numbers);

                            messages.push(ChatMessage {
                                role: Role::Function,
                                content: Some(
                                    serde_json::to_string(&random_number_result).unwrap(),
                                ),
                                name: Some("get_random_number".to_string()),
                                ..Default::default()
                            });

                            let parameters = ChatCompletionParameters {
                                model: "gpt-3.5-turbo-0613".to_string(),
                                messages: messages.clone(),
                                ..Default::default()
                            };

                            let result = block_on(client.chat().create(parameters));

                            println!("{:?}", result);
                        }
                    }
                }
            }),
            Err(e) => eprintln!("{}", e),
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
