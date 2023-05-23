use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::completion::CompletionParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CompletionParameters {
        model: "text-davinci-003".to_string(),
        prompt: "Say this is a test".to_string(),
        max_tokens: Some(10),
        ..Default::default()
    };

    let result = client.completions().create(parameters).await.unwrap();

    println!("{:?}", result);
}
