use openai_dive::v1::api::Client;
use openai_dive::v1::models::TTSEngine;
use openai_dive::v1::resources::audio::{
    AudioSpeechParameters, AudioSpeechResponseFormat, AudioVoice,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioSpeechParameters {
        model: TTSEngine::Tts1.to_string(),
        input: "Hallo, this is a test from OpenAI Dive.".to_string(),
        voice: AudioVoice::Alloy,
        response_format: Some(AudioSpeechResponseFormat::Mp3),
        speed: Some(1.0),
    };

    let response = client.audio().create_speech(parameters).await.unwrap();

    response.save("files/example.mp3").await.unwrap();
}
