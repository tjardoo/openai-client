use openai_dive::v1::api::Client;
use openai_dive::v1::resources::batch::{BatchCompletionWindow, CreateBatchParametersBuilder};
use openai_dive::v1::resources::file::{FilePurpose, UploadFileParameters};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    // We upload a file with requests for the new batch using the files endpoint.
    let parameters = UploadFileParameters {
        file: FileUpload::File("./files/demo-batch-request.jsonl".to_string()),
        purpose: FilePurpose::Batch,
    };

    let file = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", &file);

    let parameters = CreateBatchParametersBuilder::default()
        .input_file_id(file.id)
        .endpoint("/v1/chat/completions".to_string())
        .completion_window(BatchCompletionWindow::H24)
        .build()
        .unwrap();

    let result = client.batches().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
