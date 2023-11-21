use openai_dive::v1::api::Client;
use openai_dive::v1::resources::assistant::ListAssistantsParameters;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ListAssistantsParameters {
        limit: None,
        order_by: None,
        after: None,
        before: None,
    };

    let result = client.assistants().list(Some(parameters)).await.unwrap();

    println!("{:?}", result);
}
