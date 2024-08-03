use openai_dive::v1::api::Client;
use openai_dive::v1::resources::administration::invite::CreateInviteParameters;
use openai_dive::v1::resources::administration::user::UserRole;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let invites = client.administration().invites().list(None).await?;

    println!("{:#?}", invites);

    let parameters = CreateInviteParameters {
        email: "XXX".to_string(),
        role: UserRole::Reader,
    };

    let _invite = client.administration().invites().create(parameters).await?;

    let invite = client
        .administration()
        .invites()
        .retrieve("invite-XXX")
        .await?;

    println!("{:#?}", invite);

    let deleted_invite = client
        .administration()
        .invites()
        .delete("invite-XXX")
        .await?;

    println!("{:#?}", deleted_invite);

    Ok(())
}
