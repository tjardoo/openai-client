use dotenv::dotenv;
use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::assistant::{AssistantCodeInterpreterTool, AssistantParameters, AssistantTools},
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let assistant_id = env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let parameters = AssistantParameters {
        model: Gpt4Engine::Gpt41106Preview.to_string(),
        name: Some("Mathematician".to_string()),
        description: None,
        instructions: Some(
            "You are a personal math tutor. When asked a question, write and run Python code to answer the question."
                .to_string(),
        ),
        tools: Some(vec![AssistantTools::CodeInterpreter(
            AssistantCodeInterpreterTool {
                r#type: "code_interpreter".to_string(),
            }
        )]),
        file_ids: vec![],
        metadata: None,
    };

    let result = client
        .assistants()
        .modify(&assistant_id, parameters)
        .await
        .unwrap();

    println!("{:?}", result);
}
