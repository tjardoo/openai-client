use openai_dive::v1::{
    api::Client,
    resources::assistant::run::{
        CreateRunParametersBuilder, ModifyRunParametersBuilder, Run, RunStatus,
    },
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let thread_id = std::env::var("THREAD_ID").expect("THREAD_ID is not set in the .env file.");

    let assistant_id =
        std::env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let run = create_run(&client, &thread_id, &assistant_id).await;

    if run.status == RunStatus::Completed {
        modify_run(&client, &thread_id, &run.id).await;
    }

    retrieve_run(&client, &thread_id, &run.id).await;

    list_runs(&client, &thread_id).await;
}

pub async fn create_run(client: &Client, thread_id: &str, assistant_id: &str) -> Run {
    let parameters = CreateRunParametersBuilder::default()
        .assistant_id(assistant_id.to_string())
        .build()
        .unwrap();

    let run = client
        .assistants()
        .runs()
        .create(thread_id, parameters)
        .await
        .unwrap();

    run
}

pub async fn modify_run(client: &Client, thread_id: &str, run_id: &str) -> Run {
    let mut metadata = HashMap::new();

    metadata.insert("modified".to_string(), "true".to_string());
    metadata.insert("user".to_string(), "abc123".to_string());

    let parameters = ModifyRunParametersBuilder::default()
        .metadata(metadata)
        .build()
        .unwrap();

    let run = client
        .assistants()
        .runs()
        .modify(thread_id, run_id, parameters)
        .await
        .unwrap();

    run
}

pub async fn retrieve_run(client: &Client, thread_id: &str, run_id: &str) {
    let result = client
        .assistants()
        .runs()
        .retrieve(thread_id, run_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}

pub async fn list_runs(client: &Client, thread_id: &str) {
    let result = client
        .assistants()
        .runs()
        .list(thread_id, None)
        .await
        .unwrap();

    println!("{:#?}", result);
}
