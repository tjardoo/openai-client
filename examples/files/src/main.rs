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
        .purpose(FilePurpose::FineTune)
        .build()
        .unwrap();

    let file = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", &file);

    let result = client.files().retrieve(&file.id).await.unwrap();
    println!("{:#?}", result);

    let result = client.files().retrieve_content(&file.id).await.unwrap();
    println!("{:#?}", result);

    let result = client.files().list(None).await.unwrap();
    println!("{:#?}", result);

    let result = client.files().delete(&file.id).await.unwrap();
    println!("{:#?}", result);
}
