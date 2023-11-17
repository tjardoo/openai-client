use openai_dive::v1::{api::Client, resources::fine_tuning::ListFineTuningJobsParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let query = ListFineTuningJobsParameters {
        after: None,
        limit: None,
    };

    let result = client.fine_tuning().list(Some(query)).await.unwrap();

    println!("{:?}", result);
}
