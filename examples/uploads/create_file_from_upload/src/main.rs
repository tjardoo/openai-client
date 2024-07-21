use std::fs;

use openai_dive::v1::{
    api::Client,
    resources::{
        file::FilePurpose,
        shared::{FileUpload, FileUploadBytes},
        upload::{
            AddPartParametersBuilder, CompleteUploadParametersBuilder,
            CreateUploadParametersBuilder,
        },
    },
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateUploadParametersBuilder::default()
        .filename("demo-file.txt")
        .purpose(FilePurpose::FineTune)
        .bytes(fs::read("./files/demo-file.txt").unwrap().len() as u64)
        .mime_type("text/plain")
        .build()
        .unwrap();

    let upload = client.uploads().create(parameters).await.unwrap();

    println!("{:#?}", upload);

    // Add part to upload
    let parameters = AddPartParametersBuilder::default()
        .data(FileUpload::Bytes(FileUploadBytes::new(
            fs::read("./files/demo-file.txt").unwrap(),
            "demo-file.txt".to_string(),
        )))
        .build()
        .unwrap();

    let upload_part = client
        .uploads()
        .add_part(&upload.id, parameters)
        .await
        .unwrap();

    println!("{:#?}", upload_part);

    // Complete upload
    let parameters = CompleteUploadParametersBuilder::default()
        .part_ids(vec![upload_part.id])
        .build()
        .unwrap();

    let result = client
        .uploads()
        .complete(&upload.id, parameters)
        .await
        .unwrap();

    println!("{:#?}", result);

    // Cancel upload
    // let result = client
    //     .uploads()
    //     .cancel("upload_XXX")
    //     .await
    //     .unwrap();

    // println!("{:#?}", result);
}
