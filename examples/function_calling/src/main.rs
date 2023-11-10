use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{
    ChatCompletionParameters, ChatMessage, Function, Role,
};
use openai_dive::v1::resources::shared::FinishReason;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    #[derive(Serialize, Deserialize)]
    pub struct RandomNumber {
        /// Minimum value of the random number (inclusive)
        min: u32,

        /// Maximum value of the random number (inclusive)
        max: u32,
    }

    fn get_random_number(params: RandomNumber) -> Value {
        // chosen by fair dice role, guaranteed to be random
        let random = 4;
        random.into()
    }

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let mut messages = vec![ChatMessage {
        role: Role::User,
        content: "Can you get a random number between 1 and 6 please?".to_string(),
        ..Default::default()
    }];

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages: messages.clone(),
        functions: Some(vec![Function {
            name: "get_random_number".to_string(),
            description: Some("Get a random number between two values".to_string()),
            parameters: json!({
                "type": "object",
                "properties": {
                    "min": {"type": "integer", "description": "Minimum value of the random number (inclusive)"},
                    "max": {"type": "integer", "description": "Maximum value of the random number (inclusive)"},
                }
            }),
        }]),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    if let Some(choice) = result.choices.first() {
        if choice.finish_reason == Some(FinishReason::FunctionCall) {
            if let Some(function_call) = &choice.message.function_call {
                if function_call.name == "get_random_number" {
                    let random_number_params =
                        serde_json::from_str(&function_call.arguments).unwrap();
                    let random_number_result = get_random_number(random_number_params);
                    messages.push(ChatMessage {
                        role: Role::Function,
                        content: serde_json::to_string(&random_number_result).unwrap(),
                        name: Some("get_random_number".to_string()),
                        ..Default::default()
                    });

                    let parameters = ChatCompletionParameters {
                        model: "gpt-3.5-turbo-0613".to_string(),
                        messages: messages.clone(),
                        ..Default::default()
                    };

                    let result = client.chat().create(parameters).await.unwrap();

                    println!("{:?}", result);
                }
            }
        }
    }
}
