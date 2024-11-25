use openai_dive::v1::{
    api::Client,
    resources::assistant::{
        message::{MessageAttachmentBuilder, MessageRole, MessageTool},
        thread::{
            CreateThreadParametersBuilder, ModifyThreadParametersBuilder, Thread,
            ThreadMessageBuilder,
        },
    },
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let thread = create_thread(&client).await;

    modify_thread(&client, &thread.id).await;

    retrieve_thread(&client, &thread.id).await;

    delete_thread(&client, &thread.id).await;
}

pub async fn create_thread(client: &Client) -> Thread {
    let example_file = std::env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let parameters = CreateThreadParametersBuilder::default()
        .messages(vec![ThreadMessageBuilder::default()
            .content("Hello, world!".to_string())
            .role(MessageRole::User)
            .attachments(vec![MessageAttachmentBuilder::default()
                .file_id(example_file)
                .tools(vec![MessageTool::FileSearch {
                    r#type: "file_search".to_string(),
                }])
                .build()
                .unwrap()])
            .build()
            .unwrap()])
        .build()
        .unwrap();

    client
        .assistants()
        .threads()
        .create(parameters)
        .await
        .unwrap()
}

pub async fn modify_thread(client: &Client, thread_id: &str) {
    let mut metadata = HashMap::new();

    metadata.insert("modified".to_string(), "true".to_string());
    metadata.insert("user".to_string(), "abc123".to_string());

    let parameters = ModifyThreadParametersBuilder::default()
        .metadata(metadata)
        .build()
        .unwrap();

    client
        .assistants()
        .threads()
        .modify(thread_id, parameters)
        .await
        .unwrap();
}

pub async fn retrieve_thread(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .threads()
        .retrieve(thread_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}

pub async fn delete_thread(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .threads()
        .delete(thread_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}
