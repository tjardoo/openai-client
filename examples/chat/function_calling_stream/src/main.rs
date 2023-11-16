use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunctions, ChatCompletionParameters, ChatCompletionTool,
    ChatCompletionToolChoice, ChatCompletionToolChoiceFunction,
    ChatCompletionToolChoiceFunctionName, ChatCompletionToolType, ChatMessage,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let messages = vec![ChatMessage {
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
            function: ChatCompletionFunctions {
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

    // @todo handle tool calls
    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = choice.delta.content.as_ref() {
                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
