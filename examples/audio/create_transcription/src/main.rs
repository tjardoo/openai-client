use openai_dive::v1::api::Client;
use openai_dive::v1::models::TranscriptionModel;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranscriptionParametersBuilder};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = AudioTranscriptionParametersBuilder::default()
        .file(FileUpload::File("./audio/micro-machines.mp3".to_string()))
        .model(TranscriptionModel::Whisper1.to_string())
        .response_format(AudioOutputFormat::VerboseJson)
        .build()
        .unwrap();

    let result = client
        .audio()
        .create_transcription(parameters)
        .await
        .unwrap();

    println!("{:#?}", result);
}
