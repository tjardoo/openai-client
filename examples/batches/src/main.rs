use openai_dive::v1::api::Client;
use openai_dive::v1::resources::batch::{BatchCompletionWindow, CreateBatchParametersBuilder};
use openai_dive::v1::resources::file::{FilePurpose, UploadFileParameters};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

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

    let batch = client.batches().create(parameters).await.unwrap();

    println!("{:#?}", &batch);

    let result = client.batches().retrieve(&batch.id).await.unwrap();

    println!("{result:#?}");

    let result = client.batches().list(None).await.unwrap();

    println!("{result:#?}");
}
