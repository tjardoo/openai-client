use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let fine_tuning_job_id = std::env::var("FINE_TUNING_JOB_ID")
        .expect("FINE_TUNING_JOB_ID is not set in the .env file.");

    let result = client
        .fine_tuning()
        .list_job_events(&fine_tuning_job_id, None)
        .await
        .unwrap();

    println!("{:#?}", result);
}
