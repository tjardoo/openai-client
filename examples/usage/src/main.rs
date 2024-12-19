use openai_dive::v1::api::Client;
use openai_dive::v1::resources::usage::CompletionUsageParametersBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let start_time = chrono::Utc::now() - chrono::Duration::hours(1);

    let parameters = CompletionUsageParametersBuilder::default()
        .start_time(start_time.timestamp() as u32)
        .build()?;

    let result = client.usage().completions(parameters).await.unwrap();

    println!("{:#?}", result);

    Ok(())
}
