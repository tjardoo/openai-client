use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let thread_id = std::env::var("THREAD_ID").expect("THREAD_ID is not set in the .env file.");

    let run_id = std::env::var("RUN_ID").expect("RUN_ID is not set in the .env file.");

    let run_step_id =
        std::env::var("RUN_STEP_ID").expect("RUN_STEP_ID is not set in the .env file.");

    list_run_steps(&client, &thread_id, &run_id).await;

    retrieve_run_step(&client, &thread_id, &run_id, &run_step_id).await;
}

pub async fn list_run_steps(client: &Client, thread_id: &str, run_id: &str) {
    let result = client
        .assistants()
        .run_steps()
        .list(thread_id, run_id, None)
        .await
        .unwrap();

    println!("{:#?}", result);
}

pub async fn retrieve_run_step(client: &Client, thread_id: &str, run_id: &str, run_step_id: &str) {
    let result = client
        .assistants()
        .run_steps()
        .retrieve(thread_id, run_id, run_step_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}
