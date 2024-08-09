use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::{
        assistant::{
            assistant::{
                AssistantFileSearch, AssistantParametersBuilder, AssistantTool,
                AssistantToolResource, FileSearchDetails,
            },
            vector_store::CreateVectorStoreParametersBuilder,
            vector_store_file::CreateVectorStoreFileParametersBuilder,
        },
        file::{FilePurpose, UploadFileParametersBuilder},
        shared::FileUpload,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = UploadFileParametersBuilder::default()
        .file(FileUpload::File(
            "../../../openai_dive/README.md".to_string(),
        ))
        .purpose(FilePurpose::Assistants)
        .build()?;

    let file = client.files().upload(parameters).await?;

    println!("{:#?}", file);

    let parameters = CreateVectorStoreParametersBuilder::default()
        .name("OpenAI Test Vector Store 2".to_string())
        .build()?;

    let vector_store = client
        .assistants()
        .vector_stores()
        .create(parameters)
        .await?;

    println!("{:#?}", vector_store);

    let parameters = CreateVectorStoreFileParametersBuilder::default()
        .file_id(file.id)
        .build()?;

    let vector_store_file = client
        .assistants()
        .vector_store_files()
        .create(&vector_store.id, parameters)
        .await?;

    println!("{:#?}", vector_store_file);

    let vector_store_file_list = client
        .assistants()
        .vector_store_files()
        .list(&vector_store.id, None)
        .await?;

    println!("{:#?}", vector_store_file_list);

    let parameters = AssistantParametersBuilder::default()
        .name("OpenAI Dive Explainer".to_string())
        .instructions(
            "You are a developer who teaches how to use the OpenAI Dive library.".to_string(),
        )
        .model(Gpt4Engine::Gpt4O.to_string())
        .tools(vec![AssistantTool::FileSearch {
            file_search: Some(AssistantFileSearch {
                max_num_results: None,
            }),
        }])
        .tool_resources(AssistantToolResource {
            code_interpreter: None,
            file_search: Some(FileSearchDetails {
                vector_store_ids: vec![vector_store.id],
            }),
        })
        .build()?;

    let assistant = client.assistants().create(parameters).await?;

    println!("{:#?}", assistant);

    Ok(())
}
