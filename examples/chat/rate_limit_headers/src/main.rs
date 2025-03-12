use openai_dive::v1::api::Client;
use openai_dive::v1::models::FlagshipModel;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponse, ChatMessage, ChatMessageContent,
};
use openai_dive::v1::resources::shared::ResponseWrapper;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(FlagshipModel::Gpt4O.to_string())
        .messages(vec![ChatMessage::User {
            content: ChatMessageContent::Text(
                "Which 2 country has the largest population? Just give the name: population."
                    .to_string(),
            ),
            name: None,
        }])
        .max_completion_tokens(50u32)
        .build()
        .unwrap();

    let result: ResponseWrapper<ChatCompletionResponse> =
        client.chat().create_wrapped(parameters).await.unwrap();

    println!("{:#?}", result.headers);

    println!("{:#?}", result.data);
}
