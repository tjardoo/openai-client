use std::vec;

use ftail::Ftail;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::file::{FilePurpose, UploadFileParametersBuilder};
use openai_dive::v1::resources::response::request::{ResponseInput, ResponseParametersBuilder};
use openai_dive::v1::resources::response::shared::{ResponseTool, ResponseToolChoice};
use openai_dive::v1::resources::shared::FileUpload;
use openai_dive::v1::resources::vector_store::CreateVectorStoreParametersBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("results.log", false, log::LevelFilter::Trace)
        .init()?;

    let client = Client::new_from_env();

    let parameters = UploadFileParametersBuilder::default()
        .file(FileUpload::File(
            "../../../openai_dive/README.md".to_string(),
        ))
        .purpose(FilePurpose::UserData)
        .build()?;

    let file = client.files().upload(parameters).await?;

    println!("{:#?}", file);

    let parameters = CreateVectorStoreParametersBuilder::default()
        .name("OpenAI Dive Response File Search Example".to_string())
        .file_ids(vec![file.id])
        .build()?;

    let vector_store = client.vector_stores().create(parameters).await?;

    println!("{:#?}", vector_store);

    let parameters = ResponseParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .input(ResponseInput::Text(
            "Can I use the OpenAI Dive crate for other compatible APIs like Google's Gemini?"
                .to_string(),
        ))
        .tools(vec![ResponseTool::FileSearch {
            vector_store_ids: vec![vector_store.id],
            filters: None,
            max_num_results: Some(2),
            ranking_options: None,
        }])
        .tool_choice(ResponseToolChoice::FileSearch)
        .build()?;

    let result = client.responses().create(parameters).await?;

    println!("{:#?}", result);

    Ok(())
}
