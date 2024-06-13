use openai_dive::v1::{
    api::Client,
    resources::file::{FilePurpose, UploadFileParameters},
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = UploadFileParameters {
        file: "./files/DummyUsers.json".to_string(),
        purpose: FilePurpose::Assistants,
    };

    let result = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", result);
}
