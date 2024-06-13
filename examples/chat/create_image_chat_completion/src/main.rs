use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatMessageBuilder, ChatMessageContent, ImageUrl,
    ImageUrlType, Role,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::Text(
                    "What is in this image?".to_string(),
                ))
                .build()
                .unwrap(),
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(ChatMessageContent::ImageUrl(vec![ImageUrl {
                    r#type: "image_url".to_string(),
                    text: None,
                    image_url: ImageUrlType {
                        url: "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640"
                            .to_string(),
                        detail: None,
                    },
                }]))
                .build()
                .unwrap(),
        ])
        .max_tokens(50u32)
        .build()
        .unwrap();

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
