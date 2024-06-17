use std::{collections::HashMap, thread::sleep, time::Duration};

use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::assistant::{
        assistant::{
            AssistantFunction, AssistantFunctionTool, AssistantParametersBuilder, AssistantTools,
            ToolOutput, ToolOutputsParametersBuilder,
        },
        message::{CreateMessageParametersBuilder, MessageContent, MessageRole},
        run::{CreateRunParametersBuilder, RunStatus},
        thread::{CreateThreadParametersBuilder, ThreadMessageBuilder},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = "What is the conversion rate for 5000 SGD?";

    // get the OpenAI API key and project ID
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
    let project_id = std::env::var("OPENAI_PROJECT_ID");

    // create the client and set the project ID if set
    let mut client = Client::new(api_key);
    if project_id.is_ok() {
        client.set_project(&project_id.unwrap());
    }

    // create the assistant
    let parameters = AssistantParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .name("Currency Converter Assistant")
        .instructions(
            "You are a currency converter assistant. If you're asked about currency conversion us the provided get_currency_conversion function to get the conversion with the base currency being EUR. Then use that data to answer the user's question. Always mention what the equivalent in that currency of 1 EUR. And write the full name of the currency between brackets for only the first occurrences.".to_string(),
        )
        .tools(vec![AssistantTools::Function(
            AssistantFunctionTool {
                r#type: "function".to_string(),
                function: AssistantFunction {
                    name: "get_currency_conversion".to_string(),
                    description: Some("Get the currency conversion rate.".to_string()),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "amount": {"type": "number", "description": "Amount to convert."},
                            "currency": {"type": "string", "description": "Currency to convert to."},
                        },
                        "required": ["currency"],
                    }),
                },
            }
        )])
        .build()?;

    let assistant = client.assistants().create(parameters).await?;

    println!("Assistant created: {:?}", &assistant.id);

    // create a thread
    let parameters = CreateThreadParametersBuilder::default()
        .messages(vec![ThreadMessageBuilder::default()
            .content("Hello, my name is Judy.".to_string())
            .role(MessageRole::User)
            .build()?])
        .build()?;

    let thread = client.assistants().threads().create(parameters).await?;

    println!("Thread created: {:?}", &thread.id);

    // add a message with the currency conversion question
    let parameters = CreateMessageParametersBuilder::default()
        .content(question)
        .role(MessageRole::User)
        .build()?;

    let message = client
        .assistants()
        .messages()
        .create(&thread.id, parameters)
        .await?;

    println!("Message added to thread: {:?}", &message.id);

    // create a run
    let parameters = CreateRunParametersBuilder::default()
        .assistant_id(&assistant.id)
        .build()?;

    let run = client
        .assistants()
        .runs()
        .create(&thread.id, parameters)
        .await?;

    println!("Run created: {:?}", &run.id);

    // loop until the run is completed
    loop {
        let runs = client.assistants().runs().list(&thread.id, None).await?;

        let run = runs.data.first();

        if run.is_none() {
            return Ok(());
        }

        match run.unwrap().status {
            RunStatus::Queued | RunStatus::InProgress => {
                println!("Run status: {:?}", run.unwrap().status);

                sleep(Duration::from_secs(2));
            }
            RunStatus::RequiresAction => {
                println!("Run status: {:?}", run.unwrap().status);

                let run = run.unwrap();

                if run.required_action.as_ref().is_none() {
                    break;
                }

                let required_action = run.required_action.as_ref().unwrap();

                // add support for multiple tool calls
                let tool_call = required_action.submit_tool_outputs.tool_calls.first();

                if tool_call.is_none() {
                    break;
                }

                let tool_call = tool_call.unwrap();

                if tool_call.function.name == "get_currency_conversion" {
                    let formatted_arguments: CurrencyConversionInput =
                        serde_json::from_str(&tool_call.function.arguments)?;

                    let conversion = get_currency_conversion(formatted_arguments).await?;

                    let parameters = ToolOutputsParametersBuilder::default()
                        .tool_outputs(vec![ToolOutput {
                            tool_call_id: Some(tool_call.id.clone()),
                            output: Some(conversion.to_string()),
                        }])
                        .build()?;

                    let _run = client
                        .assistants()
                        .runs()
                        .submit_tool_outputs(&thread.id, &run.id, parameters)
                        .await?;

                    println!("Tool output submitted: {:?}", &tool_call.id);
                }
            }
            _ => {
                println!("Run status: {:?}", run.unwrap().status);

                break;
            }
        }
    }

    // retrieve the messages
    let messages = client
        .assistants()
        .messages()
        .list(&thread.id, None)
        .await?;

    for message in messages.data.into_iter().rev() {
        if message.role == MessageRole::Assistant {
            for item in message.content {
                match item {
                    MessageContent::Text { text, .. } => {
                        println!("Assistant: \x1b[92m{:?}\x1b[0m", text.value);
                    }
                    _ => {}
                }
            }
        } else if message.role == MessageRole::User {
            for item in message.content {
                match item {
                    MessageContent::Text { text, .. } => {
                        println!("You: \x1b[90m{:?}\x1b[0m", text.value);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CurrencyConversionInput {
    amount: f64,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrencyConversionOutput {
    amount: f64,
    base: String,
    date: String,
    rates: HashMap<String, f64>,
}

async fn get_currency_conversion(
    input: CurrencyConversionInput,
) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.frankfurter.app/latest?amount={}&from=EUR&to={}",
        input.amount, input.currency
    );

    println!("Calling API: {:?}", url);

    let client = reqwest::Client::new();

    let currency_conversion_output = client
        .get(&url)
        .send()
        .await?
        .json::<CurrencyConversionOutput>()
        .await;

    if currency_conversion_output.is_err() {
        return Err("Failed to get currency conversion".into());
    }

    let rates = currency_conversion_output.unwrap().rates;

    let rate = rates.iter().find(|(key, _)| key == &&input.currency);

    if rate.is_none() {
        return Err("Currency not found".into());
    }

    Ok(*rate.unwrap().1)
}
