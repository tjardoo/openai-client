use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessageBuilder,
    ChatMessageContent, JsonSchemaBuilder, Role,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o-2024-08-06")
        .messages(vec![
            ChatMessageBuilder::default()
                .role(Role::System)
                .content(ChatMessageContent::Text("You are a helpful math tutor. Guide the user through the solution step by step.".to_string()))
                .build()?,
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text(
                    "how can I solve 8x + 7 = -23".to_string(),
                ))
                .build()?,
        ])
        .response_format(ChatCompletionResponseFormat::JsonSchema(JsonSchemaBuilder::default()
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
        ))
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
