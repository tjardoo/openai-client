use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParameters, ChatMessage, ChatMessageContent, ImageUrl, ImageUrlType, Role,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt4O.to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("What is in this image?".to_string()),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::ImageUrl(vec![ImageUrl {
                    r#type: "image_url".to_string(),
                    text: None,
                    image_url: ImageUrlType {
                        url: "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640"
                            .to_string(),
                        detail: None,
                    },
                }]),
                ..Default::default()
            },
        ],
        max_tokens: Some(50),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
