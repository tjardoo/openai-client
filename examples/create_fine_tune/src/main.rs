use std::env;
use openai_dive::v1::{api::Client, resources::fine_tune::CreateFineTuneParameters};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateFineTuneParameters {
        training_file: "file-XXX".to_string(),
        validation_file: None,
        model: "curie".to_string(),
        //
        suffix: None,
    };

    let result = client.fine_tunes().create(parameters).await.unwrap();

    println!("{:?}", result);
}
