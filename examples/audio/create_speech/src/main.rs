use openai_dive::v1::api::Client;
use openai_dive::v1::models::TTSEngine;
use openai_dive::v1::resources::audio::{
    AudioSpeechParametersBuilder, AudioSpeechResponseFormat, AudioVoice,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = AudioSpeechParametersBuilder::default()
        .model(TTSEngine::Tts1.to_string())
        .input("Hallo, this is a test from OpenAI Dive.")
        .voice(AudioVoice::Alloy)
        .response_format(AudioSpeechResponseFormat::Mp3)
        .build()
        .unwrap();

    let response = client.audio().create_speech(parameters).await.unwrap();

    response.save("files/example.mp3").await.unwrap();
}
