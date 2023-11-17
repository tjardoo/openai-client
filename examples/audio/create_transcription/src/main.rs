use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranscriptionParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranscriptionParameters {
        file: "./audio/micro-machines.mp3".to_string(),
        model: "whisper-1".to_string(),
        language: None,
        prompt: None,
        response_format: Some(AudioOutputFormat::Text),
        temperature: None,
    };

    let result = client
        .audio()
        .create_transcription(parameters)
        .await
        .unwrap();

    println!("{:?}", result);
}
