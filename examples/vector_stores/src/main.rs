use openai_dive::v1::{
    api::Client,
    resources::{
        file::{FilePurpose, UploadFileParametersBuilder},
        shared::FileUpload,
        vector_store::CreateVectorStoreParametersBuilder,
        vector_store_file::CreateVectorStoreFileParametersBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = UploadFileParametersBuilder::default()
        .file(FileUpload::File("../../openai_dive/README.md".to_string()))
        .purpose(FilePurpose::UserData)
        .build()?;

    let file = client.files().upload(parameters).await?;

    println!("{:#?}", file);

    let parameters = CreateVectorStoreParametersBuilder::default()
        .name("OpenAI Test Vector Store 2".to_string())
        .build()?;

    let vector_store = client.vector_stores().create(parameters).await?;

    println!("{:#?}", vector_store);

    let parameters = CreateVectorStoreFileParametersBuilder::default()
        .file_id(file.id)
        .build()?;

    let vector_store_file = client
        .vector_store_files()
        .create(&vector_store.id, parameters)
        .await?;

    println!("{:#?}", vector_store_file);

    let vector_store_file_list = client
        .vector_store_files()
        .list(&vector_store.id, None)
        .await?;

    println!("{:#?}", vector_store_file_list);

    Ok(())
}
