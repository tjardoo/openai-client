use openai_dive::v1::api::Client;
use openai_dive::v1::models::O1Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatMessage, ChatMessageAudioContentPart, ChatMessageContent,
    InputAudioData,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let data = std::fs::read("example-audio.txt").unwrap();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(O1Engine::Gpt4OAudioPreview.to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    "What do you hear in this recording?".to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::AudioContentPart(vec![ChatMessageAudioContentPart {
                    r#type: "input_audio".to_string(),
                    input_audio: InputAudioData {
                        data: String::from_utf8(data).unwrap(),
                        format: "mp3".to_string(),
                    },
                }]),
                name: None,
            },
        ])
        .max_completion_tokens(50u32)
        .build()
        .unwrap();

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
