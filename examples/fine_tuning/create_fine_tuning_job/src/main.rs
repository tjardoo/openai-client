use openai_dive::v1::api::Client;
use openai_dive::v1::resources::fine_tuning::CreateFineTuningJobParametersBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let file_id = std::env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let parameters = CreateFineTuningJobParametersBuilder::default()
        .model("gpt-4o-mini-2024-07-18".to_string())
        .training_file(file_id)
        .build()
        .unwrap();

    let result = client.fine_tuning().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
