use dotenv::dotenv;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::batch::{BatchCompletionWindow, CreateBatchParameters};
use openai_dive::v1::resources::file::{FilePurpose, UploadFileParameters};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    // We upload a file with requests for the new batch using the files endpoint.
    let parameters = UploadFileParameters {
        file: "./files/demo-batch-request.jsonl".to_string(),
        purpose: FilePurpose::Batch,
    };

    let file = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", &file);

    let parameters = CreateBatchParameters {
        input_file_id: file.id,
        endpoint: "/v1/chat/completions".to_string(),
        completion_window: BatchCompletionWindow::H24,
        metadata: None,
    };

    let result = client.batches().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
