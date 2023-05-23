use std::env;
use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::completion::CompletionParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CompletionParameters {
        model: "text-davinci-003".to_string(),
        prompt: "Create an outline for an essay about Nikola Tesla and his contributions to technology:".to_string(),
        suffix: None,
        max_tokens: Some(50),
        temperature: None,
        top_p: None,
        n: None,
        logprobs: None,
        echo: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        best_of: None,
        logit_bias: None,
        user: None,
    };

    let mut stream = client.completions().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(completion_response) => completion_response.choices.iter().for_each(|choice| {
                print!("{}", choice.text);
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
