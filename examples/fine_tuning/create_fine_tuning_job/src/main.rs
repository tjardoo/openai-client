use openai_dive::v1::{api::Client, resources::fine_tuning::CreateFineTuningJobParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_name = "file-dCW7c2epbBPuSJZWbiJnrtgF";

    let parameters = CreateFineTuningJobParameters {
        model: "gpt-3.5-turbo-1106".to_string(),
        training_file: file_name.to_string(),
        hyperparameters: None,
        suffix: None,
        validation_file: None,
    };

    let result = client.fine_tuning().create(parameters).await.unwrap();

    println!("{:?}", result);
}
