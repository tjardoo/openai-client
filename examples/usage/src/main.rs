use openai_dive::v1::api::Client;
use openai_dive::v1::resources::usage::{GroupBy, ImageSource, UsageParametersBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let start_time = chrono::Utc::now() - chrono::Duration::hours(1);

    let parameters = UsageParametersBuilder::default()
        .start_time(start_time.timestamp() as u32)
        .sources(vec![ImageSource::ImageGeneration])
        .group_by(vec![GroupBy::Source, GroupBy::Size])
        .build()?;

    let images = client.usage().images(parameters.clone()).await.unwrap();

    println!("{images:#?}");

    let parameters = UsageParametersBuilder::default()
        .start_time(start_time.timestamp() as u32)
        .group_by(vec![GroupBy::LineItem])
        .build()?;

    let costs = client.usage().costs(parameters.clone()).await.unwrap();

    println!("{costs:#?}");

    Ok(())
}
