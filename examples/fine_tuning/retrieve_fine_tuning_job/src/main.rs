use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let fine_tuning_job_id = std::env::var("FINE_TUNING_JOB_ID")
        .expect("FINE_TUNING_JOB_ID is not set in the .env file.");

    let result = client
        .fine_tuning()
        .retrieve(&fine_tuning_job_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}
