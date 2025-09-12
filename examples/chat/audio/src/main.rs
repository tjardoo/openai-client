use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Model;
use openai_dive::v1::resources::chat::{
    AudioFormat, AudioParameters, ChatCompletionParametersBuilder, ChatMessage,
    ChatMessageAudioContentPart, ChatMessageContent, ChatMessageContentPart, InputAudioData, Voice,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let recording = std::fs::read("example-audio.txt").unwrap();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Model::Gpt4OAudioPreview.to_string())
        .audio(AudioParameters {
            voice: Voice::Alloy,
            format: AudioFormat::Mp3,
        })
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    "What do you hear in this recording?".to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::ContentPart(vec![ChatMessageContentPart::Audio(
                    ChatMessageAudioContentPart {
                        r#type: "input_audio".to_string(),
                        input_audio: InputAudioData {
                            data: String::from_utf8(recording).unwrap(),
                            format: "mp3".to_string(),
                        },
                    },
                )]),
                name: None,
            },
        ])
        .max_completion_tokens(50u32)
        .build()
        .unwrap();

    let result = client.chat().create(parameters).await.unwrap();

    println!("{result:#?}");
}
