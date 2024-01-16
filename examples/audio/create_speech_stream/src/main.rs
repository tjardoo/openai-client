use futures::future;
use futures::stream::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{
    AudioSpeechParameters, AudioSpeechResponseFormat, AudioVoice,
};
use std::env;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioSpeechParameters {
        model: "tts-1".to_string(),
        input: "The quick brown fox jumped over the lazy dog.".to_string(),
        voice: AudioVoice::Alloy,
        response_format: Some(AudioSpeechResponseFormat::Mp3),
        speed: Some(1.0),
    };

    let mut file = File::create("./files/example-stream.mp3").unwrap();

    let stream = client
        .audio()
        .create_speech_stream(parameters)
        .await
        .unwrap();

    stream
        .for_each(|chunk| {
            match chunk {
                Ok(chunk) => {
                    println!("Received chunk of {} bytes", chunk.bytes.len());

                    file.write(&chunk.bytes).unwrap();
                }
                Err(error) => println!("Steam error: {:?}", error),
            }

            future::ready(())
        })
        .await;
}
