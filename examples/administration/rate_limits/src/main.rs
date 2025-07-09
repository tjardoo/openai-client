use openai_dive::v1::{
    api::Client,
    resources::administration::project_rate_limit::ModifyProjectRateLimitParametersBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let parameters = ModifyProjectRateLimitParametersBuilder::default()
        .max_requests_per_1_day(2000u32)
        .build()?;

    let _ = client
        .administration()
        .project_rate_limits()
        .modify(
            "proj_gnQ3egl0t8whJIDUNP6BK5Me",
            "rl-babbage-002",
            parameters,
        )
        .await?;

    let project_rate_limits = client
        .administration()
        .project_rate_limits()
        .list("proj_gnQ3egl0t8whJIDUNP6BK5Me", None)
        .await?;

    println!("{project_rate_limits:?}");

    Ok(())
}
