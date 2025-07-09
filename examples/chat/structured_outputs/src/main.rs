use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    JsonSchemaBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o-2024-08-06")
        .messages(vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    "You are a helpful math tutor. Guide the user through the solution step by step."
                        .to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    "How can I solve 8x + 7 = -23"
                        .to_string(),
                ),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::JsonSchema {
            json_schema: JsonSchemaBuilder::default()
                .name("math_reasoning")
                .schema(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "steps": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "explanation": { "type": "string" },
                                    "output": { "type": "string" }
                                },
                                "required": ["explanation", "output"],
                                "additionalProperties": false
                            }
                        },
                        "final_answer": { "type": "string" }
                    },
                    "required": ["steps", "final_answer"],
                    "additionalProperties": false
                }))
                .strict(true)
                .build()?
            }
        )
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{result:#?}");

    Ok(())
}
