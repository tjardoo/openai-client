use openai_dive::v1::api::Client;
use openai_dive::v1::models::WhisperEngine;
use openai_dive::v1::resources::audio::{
    AudioOutputFormat, AudioTranscriptionFile, AudioTranscriptionParametersBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranscriptionParametersBuilder::default()
        .file(AudioTranscriptionFile::File(
            "./audio/micro-machines.mp3".to_string(),
        ))
        .model(WhisperEngine::Whisper1.to_string())
        .response_format(AudioOutputFormat::VerboseJson)
        .build()?;

    let result = client
        .audio()
        .create_transcription(parameters)
        .await
        .unwrap();

    println!("{:#?}", result);

    Ok(())
}
