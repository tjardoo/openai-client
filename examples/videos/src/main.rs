use openai_dive::v1::api::Client;
use openai_dive::v1::models::VideoModel;
use openai_dive::v1::resources::video::{CreateVideoParametersBuilder, VideoJobStatus, VideoSize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = CreateVideoParametersBuilder::default()
        .prompt("Wide shot of a school bus driving down a busy street, camera follows the bus as it weaves through traffic.")
        .model(VideoModel::Sora2.to_string())
        .seconds("4")
        .size(VideoSize::Size720x1280)
        .build()?;

    let result = client.videos().create(parameters).await?;

    println!("{result:#?}");

    loop {
        let video_job = client.videos().retrieve(&result.id).await?;

        if video_job.status == VideoJobStatus::Completed {
            println!("Video job completed: {video_job:#?}");

            println!(
                "Download https://api.openai.com/v1/videos/{}/content",
                video_job.id
            );

            let bytes = client.videos().retrieve_content(&video_job.id).await?;

            tokio::fs::write("files/output.mp4", &bytes).await?;

            break;
        } else {
            println!(
                "Status: {:?} with progress {}%",
                video_job.status, video_job.progress
            );

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    let result = client.videos().list(None).await?;

    println!("List of videos:");
    for video_job in result.data {
        println!(" - {}", video_job.id);
    }

    Ok(())
}
