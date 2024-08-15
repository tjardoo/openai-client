use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::endpoints::chat::RoleTrackingStream;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParametersBuilder, ChatCompletionTool,
    ChatCompletionToolType, ChatMessage, ChatMessageContent, DeltaChatMessage, DeltaFunction,
};
use openai_dive::v1::resources::shared::FinishReason;
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

    let stream = client.chat().create_stream(parameters).await.unwrap();

    let mut tracked_stream = RoleTrackingStream::new(stream);

    let mut function = DeltaFunction {
        name: None,
        arguments: None,
    };

    while let Some(response) = tracked_stream.next().await {
        match response {
            Ok(chat_chunk_response) => chat_chunk_response.choices.iter().for_each(|choice| {
                if let DeltaChatMessage::Assistant {
                    tool_calls: Some(delta_tool_calls),
                    ..
                } = &choice.delta
                {
                    function.merge(&delta_tool_calls.first().unwrap().function.clone());
                }

                match &choice.delta {
                    DeltaChatMessage::User { content, .. } => {
                        print!("{:?}", content);
                    }
                    DeltaChatMessage::System { content, .. } => {
                        print!("{:?}", content);
                    }
                    DeltaChatMessage::Assistant {
                        content: Some(chat_message_content),
                        ..
                    } => {
                        print!("{:?}", chat_message_content);
                    }
                    _ => {}
                }

                if choice.finish_reason == Some(FinishReason::ToolCalls) {
                    let name = function.name.clone().unwrap();
                    let arguments = function.arguments.clone().unwrap();

                    if name == "get_random_number" {
                        let random_numbers: RandomNumber =
                            serde_json::from_str(&arguments).unwrap();

                        println!("Min: {:?}", &random_numbers.min);
                        println!("Max: {:?}", &random_numbers.max);

                        let random_number_result = get_random_number(random_numbers);

                        println!(
                            "Random number between those numbers: {:?}",
                            random_number_result.clone()
                        );
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
