use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::assistant::assistant::{
        AssistantParametersBuilder, AssistantResponseFormat, AssistantResponseFormatType,
        AssistantResponseFormatTypeDefinition, AssistantTool,
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let assistant_id =
        std::env::var("ASSISTANT_ID").expect("ASSISTANT_ID is not set in the .env file.");

    let parameters = AssistantParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .name("Mathematician")
        .instructions(
            "You are a personal math tutor. When asked a question, write and run Rust code to answer the question."
                .to_string(),
        )
        .tools(vec![AssistantTool::CodeInterpreter])
        .response_format(AssistantResponseFormat::Format(AssistantResponseFormatType {
            r#type: AssistantResponseFormatTypeDefinition::JsonObject,
        }))
        .build()
        .unwrap();

    let result = client
        .assistants()
        .modify(&assistant_id, parameters)
        .await
        .unwrap();

    println!("{:#?}", result);
}
