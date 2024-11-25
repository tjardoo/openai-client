use openai_dive::v1::{
    api::Client,
    resources::assistant::message::{
        CreateMessageParametersBuilder, Message, MessageRole, ModifyMessageParametersBuilder,
    },
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let thread_id = std::env::var("THREAD_ID").expect("THREAD_ID is not set in the .env file.");

    let message = create_message(&client, &thread_id).await;

    modify_message(&client, &thread_id, &message.id).await;

    retrieve_message(&client, &thread_id, &message.id).await;

    list_messages(&client, &thread_id).await;
}

pub async fn create_message(client: &Client, thread_id: &str) -> Message {
    let parameters = CreateMessageParametersBuilder::default()
        .role(MessageRole::User)
        .content("How does AI work? Explain it in simple terms.".to_string())
        .build()
        .unwrap();

    let message = client
        .assistants()
        .messages()
        .create(thread_id, parameters)
        .await
        .unwrap();

    println!("{:#?}", message);

    message
}

pub async fn modify_message(client: &Client, thread_id: &str, message_id: &str) {
    let mut metadata = HashMap::new();

    metadata.insert("modified".to_string(), "true".to_string());
    metadata.insert("user".to_string(), "abc123".to_string());

    let parameters = ModifyMessageParametersBuilder::default()
        .metadata(metadata)
        .build()
        .unwrap();

    client
        .assistants()
        .messages()
        .modify(thread_id, message_id, parameters)
        .await
        .unwrap();
}

pub async fn retrieve_message(client: &Client, thread_id: &str, message_id: &str) -> Message {
    let message = client
        .assistants()
        .messages()
        .retrieve(thread_id, message_id)
        .await
        .unwrap();

    println!("{:#?}", message);

    message
}

pub async fn list_messages(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .messages()
        .list(thread_id, None)
        .await
        .unwrap();

    println!("{:#?}", result);
}
