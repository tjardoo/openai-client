use openai_dive::v1::{
    api::Client, resources::administration::audit_log::AuditLogParametersBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let parameters = AuditLogParametersBuilder::default()
        .event_types(vec![
            "login.succeeded".to_string(),
            "login.failed".to_string(),
        ])
        .limit(20u32)
        .build()?;

    let audit_logs = client
        .administration()
        .audit_logs()
        .list(Some(parameters))
        .await?;

    println!("{audit_logs:#?}");

    Ok(())
}
