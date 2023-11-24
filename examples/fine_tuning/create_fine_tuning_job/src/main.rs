use dotenv::dotenv;
use openai_dive::v1::{api::Client, resources::fine_tuning::CreateFineTuningJobParameters};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let parameters = CreateFineTuningJobParameters {
        model: "gpt-3.5-turbo-1106".to_string(),
        training_file: file_id,
        hyperparameters: None,
        suffix: None,
        validation_file: None,
    };

    let result = client.fine_tuning().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
