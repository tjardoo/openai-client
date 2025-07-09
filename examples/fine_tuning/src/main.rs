use openai_dive::v1::api::Client;
use openai_dive::v1::resources::file::{FilePurpose, UploadFileParametersBuilder};
use openai_dive::v1::resources::fine_tuning::CreateFineTuningJobParametersBuilder;
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let parameters = UploadFileParametersBuilder::default()
        .file(FileUpload::File(
            "./files/FineTuningJobSample2.jsonl".to_string(),
        ))
        .purpose(FilePurpose::FineTune)
        .build()?;

    let file = client.files().upload(parameters).await?;
    println!("{:#?}", &file);

    let parameters = CreateFineTuningJobParametersBuilder::default()
        .model("gpt-4o-mini-2024-07-18".to_string())
        .training_file(file.id)
        .build()?;

    let fine_tuning_job = client.fine_tuning().create(parameters).await?;
    println!("{fine_tuning_job:#?}");

    let fine_tuning_checkpoints = client
        .fine_tuning()
        .list_checkpoints(&fine_tuning_job.id, None)
        .await?;

    println!("{fine_tuning_checkpoints:#?}");

    let fine_tuning_events = client
        .fine_tuning()
        .list_job_events(&fine_tuning_job.id, None)
        .await?;

    println!("{fine_tuning_events:#?}");

    let result = client.fine_tuning().retrieve(&fine_tuning_job.id).await?;
    println!("{result:#?}");

    let fine_tuning_jobs = client.fine_tuning().list(None).await?;
    println!("{fine_tuning_jobs:#?}");

    Ok(())
}
