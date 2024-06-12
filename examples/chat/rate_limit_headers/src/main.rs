use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponse, ChatMessageBuilder,
    ChatMessageContent, Role,
};
use openai_dive::v1::resources::shared::ResponseWrapper;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![ChatMessageBuilder::default()
            .role(Role::User)
            .content(ChatMessageContent::Text(
                "Which 2 country has the largest population? Just give the name: population."
                    .to_string(),
            ))
            .build()?])
        .max_tokens(50u32)
        .build()?;

    let result: ResponseWrapper<ChatCompletionResponse> =
        client.chat().create_wrapped(parameters).await.unwrap();

    println!("{:#?}", result.headers);

    println!("{:#?}", result.data);

    Ok(())
}
