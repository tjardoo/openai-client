use std::env;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioTranscriptionParameters, AudioTranscriptOutputFormat};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranscriptionParameters {
        file: "./audio/micro-machines.mp3".to_string(), // https://github.com/betalgo/openai/tree/master/OpenAI.Playground/SampleData
        model: "whisper-1".to_string(),
        prompt: None,
        response_format: Some(AudioTranscriptOutputFormat::Srt),
        temperature: None,
        language: None,
    };

    let result = client.audio().create_transcription(parameters).await.unwrap();

    println!("{:?}", result);
}
