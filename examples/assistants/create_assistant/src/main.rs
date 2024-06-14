use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::assistant::assistant::AssistantCodeInterpreterTool;
use openai_dive::v1::resources::assistant::assistant::AssistantParametersBuilder;
use openai_dive::v1::resources::assistant::assistant::AssistantResponseFormat;
use openai_dive::v1::resources::assistant::assistant::AssistantTools;
use std::env;
use std::vec;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AssistantParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .name("Mathematician")
        .instructions(
            "You are a personal math tutor. When asked a question, write and run PHP code to answer the question."
                .to_string(),
        )
        .tools(vec![AssistantTools::CodeInterpreter(
            AssistantCodeInterpreterTool {
                r#type: "code_interpreter".to_string(),
            }
        )])
        .response_format(AssistantResponseFormat::JsonObject { r#type: "text".to_string() })
        .build()
        .unwrap();

    let result = client.assistants().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
