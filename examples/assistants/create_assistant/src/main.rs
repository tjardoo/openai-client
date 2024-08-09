use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::assistant::assistant::AssistantParametersBuilder;
use openai_dive::v1::resources::assistant::assistant::AssistantResponseFormat;
use openai_dive::v1::resources::assistant::assistant::AssistantResponseFormatType;
use openai_dive::v1::resources::assistant::assistant::AssistantResponseFormatTypeDefinition;
use openai_dive::v1::resources::assistant::assistant::AssistantTool;
use std::vec;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AssistantParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .name("Mathematician")
        .instructions(
            "You are a personal math tutor. When asked a question, write and run PHP code to answer the question."
                .to_string(),
        )
        .tools(vec![AssistantTool::CodeInterpreter])
        .response_format(AssistantResponseFormat::Format(AssistantResponseFormatType {
            r#type: AssistantResponseFormatTypeDefinition::Text,
        }))
        .build()
        .unwrap();

    let result = client.assistants().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
