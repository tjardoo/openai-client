use openai_dive::v1::api::Client;
use openai_dive::v1::resources::administration::project::{
    CreateProjectParameters, ModifyProjectParameters,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let admin_api_key =
        std::env::var("OPENAI_ADMIN_API_KEY").expect("OPENAI_ADMIN_API_KEY is not set");

    let client = Client::new(admin_api_key);

    let projects = client.administration().projects().list(None).await?;

    println!("{projects:#?}");

    let parameters = CreateProjectParameters {
        name: "Test Project A".to_string(),
    };

    let project = client
        .administration()
        .projects()
        .create(parameters)
        .await?;

    let project_a = client
        .administration()
        .projects()
        .retrieve(&project.id)
        .await?;

    println!("{project_a:#?}");

    let parameters = ModifyProjectParameters {
        name: "Test Project B".to_string(),
    };

    let project_b = client
        .administration()
        .projects()
        .modify(&project.id, parameters)
        .await?;

    println!("{project_b:#?}");

    let archived_project = client
        .administration()
        .projects()
        .archive(&project.id)
        .await?;

    println!("{archived_project:#?}");

    Ok(())
}
