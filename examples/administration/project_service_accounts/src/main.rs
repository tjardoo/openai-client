use openai_dive::v1::api::Client;
use openai_dive::v1::resources::administration::project_service_account::CreateProjectServiceAccountParameters;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let project_id = "proj_XXX";

    let project_service_accounts = client
        .administration()
        .project_service_accounts()
        .list(project_id, None)
        .await?;

    println!("{project_service_accounts:#?}");

    let parameters = CreateProjectServiceAccountParameters {
        name: "Test Project Service Account A".to_string(),
    };

    let _project_service_account = client
        .administration()
        .project_service_accounts()
        .create(project_id, parameters)
        .await?;

    let project_service_account = client
        .administration()
        .project_service_accounts()
        .retrieve(project_id, "user-XXX")
        .await?;

    println!("{project_service_account:#?}");

    let deleted_project_service_account = client
        .administration()
        .project_service_accounts()
        .delete(project_id, "user-XXX")
        .await?;

    println!("{deleted_project_service_account:#?}");

    Ok(())
}
