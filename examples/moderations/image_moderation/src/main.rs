use openai_dive::v1::api::Client;
use openai_dive::v1::models::ModerationModel;
use openai_dive::v1::resources::moderation::{
    ModerationInput, ModerationObject, ModerationParametersBuilder,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = ModerationParametersBuilder::default()
        .model(ModerationModel::OmniModerationLatest.to_string())
        .input(ModerationInput::MultiModal(vec![
            ModerationObject::text("I want to kill them."),
            ModerationObject::image_url("https://unsplash.com/photos/j8T2DtLTGUU/download?ixid=M3wxMjA3fDB8MXxzZWFyY2h8MjJ8fGFzaWFuJTIwYmlraW5pfGVufDB8fHx8MTcyNzM3NTQ4N3ww&force=true&w=640"), // #SFW
        ]))
        .build()
        .unwrap();

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
