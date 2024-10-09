use std::{io::Write, vec};

use futures_util::{SinkExt, TryStreamExt};
use openai_dive::v1::{
    api::Client,
    resources::realtime::{
        client::{ConversationItemCreateBuilder, ResponseCreate},
        get_realtime_server_events_deserializers,
        resources::item::{ContentType, Item, ItemContent, ItemRole, ItemType},
        server::ResponseAudioTranscriptDelta,
    },
};
use reqwest_websocket::Message;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let mut websocket = client
        .realtime()
        .websocket("gpt-4o-realtime-preview-2024-10-01")
        .await?;

    let message = ConversationItemCreateBuilder::default()
        .item(Item {
            r#type: Some(ItemType::Message),
            role: Some(ItemRole::User),
            content: Some(vec![ItemContent {
                r#type: ContentType::InputText,
                text: Some("Hello, how are you?".to_string()),
                audio: None,
                transcript: None,
            }]),
            ..Default::default()
        })
        .build()?;

    websocket
        .send(Message::Text(serde_json::to_string(&message)?))
        .await?;

    let message = ResponseCreate {
        r#type: "response.create".to_string(),
        ..Default::default()
    };

    let deserializers = get_realtime_server_events_deserializers();

    websocket
        .send(Message::Text(serde_json::to_string(&message)?))
        .await?;

    let mut has_asked_followup = false;

    while let Some(message) = websocket.try_next().await? {
        if let Message::Text(text) = message {
            match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(message_type) = json.get("type").and_then(|t| t.as_str()) {
                        if let Some(deserializer) = deserializers.get(message_type) {
                            match deserializer(&text) {
                                Ok(_struct_value) => {
                                    // skip printing the response.audio.delta
                                    if message_type != "response.audio.delta"
                                        && message_type != "response.audio_transcript.delta"
                                    {
                                        // println!("Received: {:?}", struct_value);
                                    }

                                    if message_type == "response.audio_transcript.delta" {
                                        if let Ok(response) =
                                            serde_json::from_str::<ResponseAudioTranscriptDelta>(
                                                &text,
                                            )
                                        {
                                            print!("{}", response.delta);
                                            std::io::stdout().flush()?;
                                        }
                                    } else if message_type == "response.audio_transcript.done" {
                                        println!();

                                        if !has_asked_followup {
                                            let followup_message = ConversationItemCreateBuilder::default()
                                                .item(Item {
                                                    r#type: Some(ItemType::Message),
                                                    role: Some(ItemRole::User),
                                                    content: Some(vec![ItemContent {
                                                        r#type: ContentType::InputText,
                                                        text: Some("Good! What's the weather usually in Bangkok in the October?".to_string()),
                                                        audio: None,
                                                        transcript: None,
                                                    }]),
                                                    ..Default::default()
                                                })
                                                .build()?;

                                            websocket
                                                .send(Message::Text(serde_json::to_string(
                                                    &followup_message,
                                                )?))
                                                .await?;

                                            let message = ResponseCreate {
                                                r#type: "response.create".to_string(),
                                                ..Default::default()
                                            };

                                            websocket
                                                .send(Message::Text(serde_json::to_string(
                                                    &message,
                                                )?))
                                                .await?;

                                            has_asked_followup = true;
                                        }
                                    }
                                }
                                Err(error) => {
                                    eprintln!("Failed to deserialize {}: {}", message_type, error);
                                }
                            }
                        } else {
                            println!("Unknown message type: {}", message_type);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Failed to deserialize message: {}", error);
                }
            }
        }
    }

    Ok(())
}
