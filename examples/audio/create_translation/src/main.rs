use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranslationParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranslationParameters {
        file: "./audio/multilingual.mp3".to_string(),
        model: "whisper-1".to_string(),
        prompt: None,
        response_format: Some(AudioOutputFormat::Srt),
        temperature: None,
    };

    let result = client.audio().create_translation(parameters).await.unwrap();

    println!("{:?}", result);
}
