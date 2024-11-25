use futures::future;
use futures::stream::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::models::TTSEngine;
use openai_dive::v1::resources::audio::{
    AudioSpeechParametersBuilder, AudioSpeechResponseFormat, AudioVoice,
};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = AudioSpeechParametersBuilder::default()
        .model(TTSEngine::Tts1.to_string())
        .input("The quick brown fox jumped over the lazy dog.")
        .voice(AudioVoice::Alloy)
        .response_format(AudioSpeechResponseFormat::Mp3)
        .build()
        .unwrap();

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

                    file.write_all(&chunk.bytes).unwrap();
                }
                Err(error) => println!("Steam error: {:?}", error),
            }

            future::ready(())
        })
        .await;
}
