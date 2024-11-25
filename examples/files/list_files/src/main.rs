use openai_dive::v1::{
    api::Client,
    resources::file::{FilePurpose, ListFilesParameters},
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let query = ListFilesParameters {
        purpose: Some(FilePurpose::FineTune),
    };

    let result = client.files().list(Some(query)).await.unwrap();

    println!("{:#?}", result);
}
