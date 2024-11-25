use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Client::new_from_env();

    let assistant_id =
        std::env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let result = client.assistants().delete(&assistant_id).await.unwrap();

    println!("{:#?}", result);
}
