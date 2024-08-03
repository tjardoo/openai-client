use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let project_id = "proj_XXX";

    let project_api_keys = client
        .administration()
        .project_api_keys()
        .list(project_id, None)
        .await?;

    println!("{:#?}", project_api_keys);

    let project_api_key = client
        .administration()
        .project_api_keys()
        .retrieve(project_id, "key_XXX")
        .await?;

    println!("{:#?}", project_api_key);

    let deleted_project_api_key = client
        .administration()
        .project_api_keys()
        .delete(project_id, "key_XXX")
        .await?;

    println!("{:#?}", deleted_project_api_key);

    Ok(())
}
