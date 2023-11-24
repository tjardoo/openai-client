use dotenv::dotenv;
use openai_dive::v1::{
    api::Client,
    resources::assistant::{AssistantFile, CreateAssistantFileParameters},
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    create_assistant_file(&client, &assistant_id, &file_id).await;

    retrieve_assistant_file(&client, &assistant_id, &file_id).await;

    list_assistant_files(&client, &assistant_id).await;

    delete_assistant_file(&client, &assistant_id, &file_id).await;
}

pub async fn create_assistant_file(
    client: &Client,
    assistant_id: &str,
    file_id: &str,
) -> AssistantFile {
    let parameters = CreateAssistantFileParameters {
        file_id: file_id.to_string(),
    };

    let assistant_file = client
        .assistants()
        .files()
        .create(&assistant_id, parameters)
        .await
        .unwrap();

    assistant_file
}

pub async fn retrieve_assistant_file(
    client: &Client,
    assistant_id: &str,
    file_id: &str,
) -> AssistantFile {
    let assistant_file = client
        .assistants()
        .files()
        .retrieve(&assistant_id, &file_id)
        .await
        .unwrap();

    println!("{:?}", assistant_file);

    assistant_file
}

pub async fn list_assistant_files(client: &Client, assistant_id: &str) {
    let result = client
        .assistants()
        .files()
        .list(&assistant_id, None)
        .await
        .unwrap();

    println!("{:?}", result);
}

pub async fn delete_assistant_file(client: &Client, assistant_id: &str, file_id: &str) {
    let result = client
        .assistants()
        .files()
        .delete(&assistant_id, &file_id)
        .await
        .unwrap();

    println!("{:?}", result);
}
