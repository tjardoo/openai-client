use std::env;
use openai_dive::v1::{api::Client, resources::file::UploadFileParameters};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = UploadFileParameters {
        file: "./files/SentimentAnalysisSample.jsonl".to_string(), // https://github.com/betalgo/openai/tree/master/OpenAI.Playground/SampleData
        purpose: "fine-tune".to_string(),
    };

    let result = client.files().upload(parameters).await.unwrap();

    println!("{:?}", result);
}
