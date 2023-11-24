use dotenv::dotenv;
use openai_dive::v1::{
    api::Client,
    resources::assistant_resources::thread::{
        CreateThreadParameters, ModifyThreadParameters, Thread, ThreadMessage, ThreadMessageRole,
    },
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let thread = create_thread(&client).await;

    let thread = modify_thread(&client, &thread.id).await;

    retrieve_thread(&client, &thread.id).await;

    delete_thread(&client, &thread.id).await;
}

pub async fn create_thread(client: &Client) -> Thread {
    let parameters = CreateThreadParameters {
        messages: Some(vec![ThreadMessage {
            role: ThreadMessageRole::User,
            content: "None".to_string(),
            file_ids: None,
            metadata: None,
        }]),
        metadata: None,
    };

    let thread = client
        .assistants()
        .threads()
        .create(parameters)
        .await
        .unwrap();

    thread
}

pub async fn modify_thread(client: &Client, thread_id: &str) -> Thread {
    let mut metadata = HashMap::new();

    metadata.insert("modified".to_string(), "true".to_string());
    metadata.insert("user".to_string(), "abc123".to_string());

    let parameters = ModifyThreadParameters {
        metadata: Some(metadata),
    };

    let thread = client
        .assistants()
        .threads()
        .modify(thread_id, parameters)
        .await
        .unwrap();

    thread
}

pub async fn retrieve_thread(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .threads()
        .retrieve(&thread_id)
        .await
        .unwrap();

    println!("{:?}", result);
}

pub async fn delete_thread(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .threads()
        .delete(&thread_id)
        .await
        .unwrap();

    println!("{:?}", result);
}
