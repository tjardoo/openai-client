use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponse, ChatMessageBuilder,
    ChatMessageContent, Role,
};
use openai_dive::v1::resources::shared::ResponseWrapper;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![ChatMessageBuilder::default()
            .role(Role::User)
            .content(ChatMessageContent::Text(
                "Which 2 country has the largest population? Just give the name: population."
                    .to_string(),
            ))
            .build()
            .unwrap()])
        .max_tokens(50u32)
        .build()
        .unwrap();

    let result: ResponseWrapper<ChatCompletionResponse> =
        client.chat().create_wrapped(parameters).await.unwrap();

    println!("{:#?}", result.headers);

    println!("{:#?}", result.data);
}
