use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::assistant::{
        assistant::{AssistantParametersBuilder, AssistantTool},
        run::{CreateRunParametersBuilder, CreateThreadAndRunParametersBuilder},
        thread::{CreateThreadParametersBuilder, ThreadMessageBuilder},
    },
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AssistantParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .name("Rust Mathematician".to_string())
        .instructions(
            "You are a personal math tutor. When asked a question, write and run Rust code to answer the question."
                .to_string(),
        )
        .tools(vec![AssistantTool::CodeInterpreter])
        .build()
        .unwrap();

    let assistant = client.assistants().create(parameters).await.unwrap();

    let parameters = CreateThreadAndRunParametersBuilder::default()
        .thread(
            CreateThreadParametersBuilder::default()
                .messages(vec![ThreadMessageBuilder::default()
                    .content("What is the square root of 25?".to_string())
                    .build()
                    .unwrap()])
                .build()
                .unwrap(),
        )
        .run(
            CreateRunParametersBuilder::default()
                .assistant_id(assistant.id)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let run = client
        .assistants()
        .runs()
        .create_thread_and_run(parameters)
        .await
        .unwrap();

    println!("{:#?}", run);
}
