use std::{collections::HashMap, thread::sleep, time::Duration};

use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::assistant::{
        assistant::{
            AssistantFunction, AssistantFunctionTool, AssistantParametersBuilder, AssistantTools,
            ToolOutput, ToolOutputsParametersBuilder,
        },
        message::{MessageContent, MessageRole},
        run::{CreateRunParametersBuilder, CreateThreadAndRunParametersBuilder, RunStatus},
        thread::{CreateThreadParametersBuilder, ThreadMessageBuilder},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = "What is the conversion rate for 5000 SGD? And 1000 JPY?";

    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
    let project_id = std::env::var("OPENAI_PROJECT_ID");

    let mut client = Client::new(api_key);
    if project_id.is_ok() {
        client.set_project(&project_id.unwrap());
    }

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

    let parameters = CreateThreadAndRunParametersBuilder::default()
        .thread(
            CreateThreadParametersBuilder::default()
                .messages(vec![
                    ThreadMessageBuilder::default()
                        .content("Hello, my name is Judy.".to_string())
                        .build()?,
                    ThreadMessageBuilder::default().content(question).build()?,
                ])
                .build()?,
        )
        .run(
            CreateRunParametersBuilder::default()
                .assistant_id(assistant.id)
                .build()?,
        )
        .build()?;

    let run = client
        .assistants()
        .runs()
        .create_thread_and_run(parameters)
        .await?;

    println!("Thread and run created: {:?}", &run.id);

    let mut tool_outputs = Vec::<ToolOutput>::new();

    loop {
        let runs = client
            .assistants()
            .runs()
            .list(&run.thread_id, None)
            .await?;

        let run = runs.data.first();

        if run.is_none() {
            return Ok(());
        }

        let run = run.unwrap();

        match run.status {
            RunStatus::Queued | RunStatus::InProgress => {
                println!("Run status: {:?}", run.status);

                sleep(Duration::from_secs(2));
            }
            RunStatus::RequiresAction => {
                println!("Run status: {:?}", run.status);

                if run.required_action.as_ref().is_none() {
                    break;
                }

                let required_action = run.required_action.as_ref().unwrap();

                for tool_call in &required_action.submit_tool_outputs.tool_calls {
                    if tool_call.function.name == "get_currency_conversion" {
                        let formatted_arguments: CurrencyConversionInput =
                            serde_json::from_str(&tool_call.function.arguments)?;

                        let conversion = get_currency_conversion(formatted_arguments).await?;

                        tool_outputs.push(ToolOutput {
                            tool_call_id: Some(tool_call.id.clone()),
                            output: Some(conversion.to_string()),
                        });
                    }
                }

                break;
            }
            _ => {
                println!("Run status: {:?}", run.status);

                break;
            }
        }
    }

    let parameters = ToolOutputsParametersBuilder::default()
        .tool_outputs(tool_outputs)
        .build()?;

    let run = client
        .assistants()
        .runs()
        .submit_tool_outputs(&run.thread_id, &run.id, parameters)
        .await?;

    println!("Tool output submitted: {:?}", &run.id);

    // wait for the messages to be processed
    sleep(Duration::from_secs(5));

    let messages = client
        .assistants()
        .messages()
        .list(&run.thread_id, None)
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
