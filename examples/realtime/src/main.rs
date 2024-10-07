use futures_util::{SinkExt, TryStreamExt};
use openai_dive::v1::{
    api::Client,
    resources::realtime::{
        client::{ConversationItemCreateBuilder, ResponseCreate},
        get_realtime_server_events_deserializers,
        server::ResponseAudioTranscriptDelta,
        shared::{Content, ContentType, ItemBuilder, ItemRole, ItemType},
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
        .init("gpt-4o-realtime-preview-2024-10-01")
        .await?;

    let message = ConversationItemCreateBuilder::default()
        .item(
            ItemBuilder::default()
                .r#type(ItemType::Message)
                .role(ItemRole::User)
                .content(vec![Content {
                    r#type: ContentType::InputText,
                    text: Some("Hello, how are you?".to_string()),
                    ..Default::default()
                }])
                .build()?,
        )
        .build()?;

    websocket
        .send(Message::Text(serde_json::to_string(&message).unwrap()))
        .await
        .unwrap();

    let message = ResponseCreate {
        r#type: "response.create".to_string(),
        ..Default::default()
    };

    let deserializers = get_realtime_server_events_deserializers();

    websocket
        .send(Message::Text(serde_json::to_string(&message).unwrap()))
        .await
        .unwrap();

    while let Some(message) = websocket.try_next().await.unwrap() {
        if let Message::Text(text) = message {
            match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(message_type) = json.get("type").and_then(|t| t.as_str()) {
                        if let Some(deserializer) = deserializers.get(message_type) {
                            match deserializer(&text) {
                                Ok(_struct_value) => {
                                    // println!("Received: {:?}", struct_value)

                                    if message_type == "response.audio_transcript.delta" {
                                        if let Ok(response) =
                                            serde_json::from_str::<ResponseAudioTranscriptDelta>(
                                                &text,
                                            )
                                        {
                                            print!("{}", response.delta);
                                        }
                                    } else if message_type == "response.audio_transcript.done" {
                                        println!();
                                    }
                                }
                                Err(_error) => {
                                    // eprintln!("Failed to deserialize {}: {}", message_type, error);
                                }
                            }
                        } else {
                            // println!("Unknown message type: {}", message_type);
                        }
                    }
                }
                Err(_error) => {
                    // eprintln!("Failed to deserialize message: {}", error);
                }
            }
        }
    }

    Ok(())
}
