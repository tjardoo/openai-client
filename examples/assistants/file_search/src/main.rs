use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::{
        assistant::{
            assistant::{
                AssistantFileSearch, AssistantParametersBuilder, AssistantTool,
                AssistantToolResource, FileSearchDetails,
            },
            message::MessageRole,
            run::{CreateRunParametersBuilder, CreateThreadAndRunParametersBuilder},
            thread::{CreateThreadParametersBuilder, ThreadMessageBuilder},
        },
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

    let parameters = CreateThreadAndRunParametersBuilder::default()
        .thread(
            CreateThreadParametersBuilder::default()
                .messages(vec![ThreadMessageBuilder::default()
                    .role(MessageRole::User)
                    .content("How can I set the correct project ID on the client?".to_string())
                    .build()?])
                .build()
                .unwrap(),
        )
        .run(
            CreateRunParametersBuilder::default()
                .assistant_id(assistant.id)
                .build()?,
        )
        .build()?;

    let run = client
        .assistants()
        .runs()
        .create_thread_and_run(parameters)
        .await?;

    println!("{:#?}", run);

    // sleep for a bit
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let messages = client
        .assistants()
        .messages()
        .list(&run.thread_id, None)
        .await?;

    println!("{:#?}", messages);

    Ok(())
}
