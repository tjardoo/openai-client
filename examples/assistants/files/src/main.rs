use dotenv::dotenv;
use openai_dive::v1::{api::Client, resources::assistant::CreateAssistantFileParameters};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    create_assistant_file(&client).await;

    retrieve_assistant_file(&client).await;

    list_assistant_files(&client).await;

    delete_assistant_file(&client).await;
}

pub async fn create_assistant_file(client: &Client) {
    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let parameters = CreateAssistantFileParameters {
        file_id: env::var("FILE_ID").expect("FILE_ID is not set in the .env file."),
    };

    let result = client
        .assistants()
        .create_file(&assistant_id, parameters)
        .await
        .unwrap();

    println!("{:?}", result);
}

pub async fn retrieve_assistant_file(client: &Client) {
    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client
        .assistants()
        .retrieve_file(&assistant_id, &file_id)
        .await
        .unwrap();

    println!("{:?}", result);
}

pub async fn list_assistant_files(client: &Client) {
    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let result = client
        .assistants()
        .list_files(&assistant_id, None)
        .await
        .unwrap();

    println!("{:?}", result);
}

pub async fn delete_assistant_file(client: &Client) {
    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client
        .assistants()
        .delete_file(&assistant_id, &file_id)
        .await
        .unwrap();

    println!("{:?}", result);
}
