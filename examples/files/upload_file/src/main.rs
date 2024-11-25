use openai_dive::v1::{
    api::Client,
    resources::{
        file::{FilePurpose, UploadFileParametersBuilder},
        shared::FileUpload,
    },
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = UploadFileParametersBuilder::default()
        .file(FileUpload::File("./files/DummyUsers.json".to_string()))
        .purpose(FilePurpose::Assistants)
        .build()
        .unwrap();

    let result = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", result);
}
