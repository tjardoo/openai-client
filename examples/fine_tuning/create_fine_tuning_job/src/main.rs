use openai_dive::v1::models::Gpt35Engine;
use openai_dive::v1::{api::Client, resources::fine_tuning::CreateFineTuningJobParameters};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = std::env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let parameters = CreateFineTuningJobParameters {
        model: Gpt35Engine::Gpt35Turbo.to_string(),
        training_file: file_id,
        hyperparameters: None,
        suffix: None,
        validation_file: None,
        integrations: None,
        seed: None,
    };

    let result = client.fine_tuning().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
