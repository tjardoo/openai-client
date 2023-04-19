use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::edit::EditParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditParameters {
        model: "text-davinci-edit-001".to_string(),
        input: "What day of the wek is it?".to_string(),
        instruction: "Fix the spelling mistakes".to_string(),
        temperature: None,
    };

    let result = client.edits().create(parameters).await.unwrap();

    println!("{:?}", result);
}
