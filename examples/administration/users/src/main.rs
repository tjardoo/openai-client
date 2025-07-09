use openai_dive::v1::api::Client;
use openai_dive::v1::resources::administration::user::{ModifyUserParameters, UserRole};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let users = client.administration().users().list(None).await?;

    println!("{users:#?}");

    let user = client.administration().users().retrieve("user-XXX").await?;

    println!("{user:#?}");

    let parameters = ModifyUserParameters {
        role: UserRole::Owner,
    };

    let modified_user = client
        .administration()
        .users()
        .modify("user-XXX", parameters)
        .await?;

    println!("{modified_user:#?}");

    Ok(())
}
