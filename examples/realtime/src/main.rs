use base64::{engine::general_purpose, Engine};
use ftail::ansi_escape::TextStyling;
use futures_util::{SinkExt, StreamExt};
#[cfg(feature = "audio")]
use hound::{WavSpec, WavWriter};
use openai_dive::v1::{
    api::Client,
    resources::realtime::{
        client::{ConversationItemCreateBuilder, ResponseCreateBuilder},
        get_realtime_server_events_deserializers,
        resources::item::{ContentType, Item, ItemContent, ItemRole, ItemType},
        server::{ResponseAudioDelta, ResponseAudioTranscriptDelta},
    },
};
use reqwest_websocket::Message;
#[cfg(feature = "audio")]
use rodio::{Decoder, OutputStream, Source};
use serde_json::Value;
use std::{io::Write, vec};
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ftail::Ftail::new()
    //     .console(log::LevelFilter::Debug)
    //     .init()?;

    let client = Client::new_from_env();

    let model = "gpt-4o-realtime-preview-2024-10-01";

    let websocket = client.realtime().websocket(model).await?;

    let deserializers = get_realtime_server_events_deserializers();

    let (mut websocket_writer, mut websocket_reader) = websocket.split();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    println!("Starting conversation with model: {}", model.bold());

    print!("{}", "You: ".black());
    std::io::stdout().flush().unwrap();

    // receive messages
    tokio::spawn(async move {
        while let Some(message) = websocket_reader.next().await {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str::<Value>(&text) {
                    Ok(json) => {
                        if let Some(message_type) = json.get("type").and_then(|t| t.as_str()) {
                            if let Some(deserializer) = deserializers.get(message_type) {
                                match deserializer(&text) {
                                    Ok(_) => {
                                        if message_type == "response.audio_transcript.delta" {
                                            if let Ok(response_audio_transcript_delta) =
                                                serde_json::from_str::<ResponseAudioTranscriptDelta>(
                                                    &text,
                                                )
                                            {
                                                print!("{}", response_audio_transcript_delta.delta);
                                                std::io::stdout().flush().unwrap();
                                            }
                                        } else if message_type == "response.done" {
                                            print!("\n{}", "You: ".black());
                                            std::io::stdout().flush().unwrap();

                                            #[cfg(feature = "audio")]
                                            {
                                                let (_stream, stream_handle) =
                                                    OutputStream::try_default().unwrap();

                                                let file =
                                                    std::fs::File::open("output.wav").unwrap();

                                                let wav_reader =
                                                    hound::WavReader::new(file).unwrap();

                                                let duration = wav_reader.len() as f64 / 24000_f64;

                                                let file =
                                                    std::fs::File::open("output.wav").unwrap();

                                                let source = Decoder::new(file).unwrap();

                                                let _ = stream_handle
                                                    .play_raw(source.convert_samples());

                                                std::thread::sleep(
                                                    std::time::Duration::from_secs_f64(duration),
                                                );
                                            }
                                        } else if message_type == "response.created" {
                                            print!("{}", "AI: ".blue());

                                            std::fs::write("output.wav", vec![]).unwrap();
                                        } else if message_type == "session.created" {
                                            // update the settings...
                                            //
                                            //
                                        } else if message_type == "response.audio.delta" {
                                            if let Ok(response_audio_delta) =
                                                serde_json::from_str::<ResponseAudioDelta>(&text)
                                            {
                                                let decoded_audio = general_purpose::STANDARD
                                                    .decode(response_audio_delta.delta.as_bytes())
                                                    .unwrap();

                                                let mut file = std::fs::OpenOptions::new()
                                                    .create(true)
                                                    .append(true)
                                                    .open("output.wav")
                                                    .unwrap();

                                                file.write_all(&decoded_audio).unwrap();
                                            }
                                        } else if message_type == "response.audio.done" {
                                            #[cfg(feature = "audio")]
                                            {
                                                let audio = std::fs::read("output.wav").unwrap();

                                                let pcm_samples: Vec<i16> = audio
                                                    .chunks(2)
                                                    .map(|chunk| {
                                                        i16::from_le_bytes([chunk[0], chunk[1]])
                                                    })
                                                    .collect();

                                                save_as_wav(&pcm_samples, 24000, "output.wav")
                                                    .unwrap();
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        eprintln!("Failed to deserialize message: {}", error);
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to deserialize message: {}", error);
                    }
                },
                Err(error) => {
                    eprintln!("Failed to deserialize message: {}", error);
                }
                _ => {}
            }
        }
    });

    // send messages
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(error) = websocket_writer.send(Message::Text(message)).await {
                eprintln!("Failed to send message: {}", error);
            }
        }
    });

    // read user input and send messages
    tokio::spawn(async move {
        let stdin = BufReader::new(tokio::io::stdin());
        let mut lines = stdin.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let trimmed_input = line.trim();

            if trimmed_input.eq_ignore_ascii_case("exit") {
                break;
            }

            let message = ConversationItemCreateBuilder::default()
                .item(Item {
                    r#type: Some(ItemType::Message),
                    role: Some(ItemRole::User),
                    content: Some(vec![ItemContent {
                        r#type: ContentType::InputText,
                        text: Some(trimmed_input.to_string()),
                        audio: None,
                        transcript: None,
                    }]),
                    ..Default::default()
                })
                .build();

            match message {
                Ok(conversation_item_create) => {
                    tx.send(serde_json::to_string(&conversation_item_create).unwrap())
                        .await
                        .unwrap();

                    let response_create = ResponseCreateBuilder::default().build().unwrap();

                    if let Err(error) = tx
                        .send(serde_json::to_string(&response_create).unwrap())
                        .await
                    {
                        eprintln!("Failed to send message: {}", error);
                    }
                }
                Err(error) => {
                    eprintln!("Failed to build message: {}", error);
                }
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");

    Ok(())
}

#[cfg(feature = "audio")]
fn save_as_wav(
    pcm_data: &[i16],
    sample_rate: u32,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(file_path, spec)?;

    for sample in pcm_data {
        writer.write_sample(*sample)?;
    }

    writer.finalize()?;

    Ok(())
}
