//! # Getting started
//!
//! OpenAI Dive is an unofficial async Rust library that allows you to interact with the OpenAI API.
//!
//! Sign up for an account on [https://platform.openai.com/overview](https://platform.openai.com/overview) to get your API key.
//!
//! ```ini
//! [dependencies]
//! openai_dive = "0.2"
//! ```
//!
//! More information: [set API key](#set-api-key), [add proxy](#add-proxy), [use model names](#use-model-names)
//!
//! ## Endpoints
//!
//! - Models
//!   - [List models](#list-models)
//!   - [Retrieve model](#retrieve-model)
//! - Chat
//!   - [Create chat completion](#create-chat-completion)
//!   - [Create chat completion (stream)](#create-chat-completion-stream)
//!   - [Calling Functions](#calling-functions)
//!   - [Calling Functions (stream)](#calling-functions-stream)
//! - Images
//!   - [Create image](#create-image)
//!   - [Create image edit](#create-image-edit)
//!   - [Create image variation](#create-image-variation)
//! - Embeddings
//!   - [Create embedding](#create-embedding)
//! - Audio
//!   - [Create transcription](#create-transcription)
//!   - [Create translation](#create-translation)
//! - Files
//!   - [List files](#list-files)
//!   - [Upload file](#upload-file)
//!   - [Delete file](#delete-file)
//!   - [Retrieve file](#retrieve-file)
//!   - [Retrieve file content](#retrieve-file-content)
//! - [Fine-tuning](#fine-tuning)
//! - Moderations
//!   - [Create moderation](#create-moderation)
//!
//! # Endpoints
//!
//! ## List models
//!
//! Lists the currently available models, and provides basic information about each one such as the owner and availability.
//!
//! **URL** `https://api.openai.com/v1/models`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.models().list().await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [List models](https://platform.openai.com/docs/api-reference/models/list)
//!
//! ## Retrieve model
//!
//! Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
//!
//! **URL** `https://api.openai.com/v1/models/{model}`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.models().get("text-davinci-003").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Retrieve models](https://platform.openai.com/docs/api-reference/models/retrieve)
//!
//! ## Create chat completion
//!
//! Creates a completion for the chat message.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0613".to_string(),
//!         messages: vec![
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Hello!".to_string(),
//!                 ..Default::default()
//!             },
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Where are you located?".to_string(),
//!                 ..Default::default()
//!             },
//!         ],
//!         temperature: None,
//!         top_p: None,
//!         n: None,
//!         stop: None,
//!         max_tokens: Some(12),
//!         presence_penalty: None,
//!         frequency_penalty: None,
//!         logit_bias: None,
//!         user: None,
//!         ..Default::default()
//!     };
//!
//!     let result = client.chat().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Create chat completion (stream)
//!
//! Creates a completion for the chat message.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use futures::StreamExt;
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0301".to_string(),
//!         messages: vec![
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Hello!".to_string(),
//!                 ..Default::default()
//!             },
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Where are you located?".to_string(),
//!                 ..Default::default()
//!             },
//!         ],
//!         temperature: None,
//!         top_p: None,
//!         n: None,
//!         stop: None,
//!         max_tokens: Some(12),
//!         presence_penalty: None,
//!         frequency_penalty: None,
//!         logit_bias: None,
//!         user: None,
//!         ..Default::default()
//!     };
//!
//!     let mut stream = client.chat().create_stream(parameters).await.unwrap();
//!
//!     while let Some(response) = stream.next().await {
//!         match response {
//!             Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
//!                 if let Some(content) = choice.delta.content.as_ref() {
//!                     print!("{}", content);
//!                 }
//!             }),
//!             Err(e) => eprintln!("{}", e),
//!         }
//!     }
//! }
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Calling Functions
//!
//! In an API call, you can describe functions and have the model intelligently choose to output a JSON object containing arguments to call one or many functions.
//! The Chat Completions API does not call the function; instead, the model generates JSON that you can use to call the function in your code.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::chat_completion::{
//!     ChatCompletionParameters, ChatMessage, Function, Role,
//! };
//! use openai_dive::v1::resources::shared::FinishReason;
//! use serde::{Deserialize, Serialize};
//! use serde_json::{json, Value};
//!
//! #[tokio::main]
//! async fn main() {
//!     #[derive(Serialize, Deserialize)]
//!     pub struct RandomNumber {
//!         min: u32, // minimum value of the random number
//!         max: u32, // maximum value of the random number
//!     }
//!
//!     fn get_random_number(params: RandomNumber) -> Value {
//!         let random = 4;
//!         random.into()
//!     }
//!
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let mut messages = vec![ChatMessage {
//!         role: Role::User,
//!         content: "Can you get a random number between 1 and 6 please?".to_string(),
//!         ..Default::default()
//!     }];
//!
//!     let parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0613".to_string(),
//!         messages: messages.clone(),
//!         functions: Some(vec![Function {
//!             name: "get_random_number".to_string(),
//!             description: Some("Get a random number between two values".to_string()),
//!             parameters: json!({
//!                 "type": "object",
//!                 "properties": {
//!                     "min": {"type": "integer", "description": "Minimum value of the random number (inclusive)"},
//!                     "max": {"type": "integer", "description": "Maximum value of the random number (inclusive)"},
//!                 }
//!             }),
//!         }]),
//!         ..Default::default()
//!     };
//!
//!     let result = client.chat().create(parameters).await.unwrap();
//!
//!     if let Some(choice) = result.choices.first() {
//!         if choice.finish_reason == Some(FinishReason::FunctionCall) {
//!             if let Some(function_call) = &choice.message.function_call {
//!                 if function_call.name == "get_random_number" {
//!                     let random_number_params =
//!                         serde_json::from_str(&function_call.arguments).unwrap();
//!                     let random_number_result = get_random_number(random_number_params);
//!                     messages.push(ChatMessage {
//!                         role: Role::Function,
//!                         content: serde_json::to_string(&random_number_result).unwrap(),
//!                         name: Some("get_random_number".to_string()),
//!                         ..Default::default()
//!                     });
//!
//!                     let parameters = ChatCompletionParameters {
//!                         model: "gpt-3.5-turbo-0613".to_string(),
//!                         messages: messages.clone(),
//!                         ..Default::default()
//!                     };
//!
//!                     let result = client.chat().create(parameters).await.unwrap();
//!
//!                     println!("{:?}", result);
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! More information: [Function calling](https://platform.openai.com/docs/guides/function-calling)
//!
//! ### Function tips
//!
//! 1. Use the [async-recursion](https://crates.io/crates/async-recursion) crate to recursively call into a wrapping function after processing a function call.
//! 2. Use the [schemars](https://crates.io/crates/schemars) crate for automatically generating JSON schemas from structs.
//!
//!    ```rust
//!    let random_number_schema = schemars::schema_for!(RandomNumber);
//!    // ...
//!
//!    Function {
//!       name: "get_random_number".to_string(),
//!       parameters: serde_json::to_value(random_number_schema).unwrap(),
//!       // ...
//!    }
//!    ```
//!
//! 3. After a function is called, stop the agent from calling more functions (sometimes it can get stuck in a function calling loop).
//!
//!    ```rust
//!    let parameters = ChatCompletionParameters {
//!        // ...
//!        function_call: Some(FunctionCallConfig::None)
//!    }
//!    ```
//!
//! ## Calling Functions (stream)
//!
//! Providing functions for the agent to call and interpret results
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use futures::StreamExt;
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::chat_completion::{
//!     ChatCompletionParameters, ChatMessage, Function, FunctionCall, Role,
//! };
//! use openai_dive::v1::resources::shared::FinishReason;
//! use serde::{Deserialize, Serialize};
//! use serde_json::{json, Value};
//!
//! #[tokio::main]
//! async fn main() {
//!     #[derive(Serialize, Deserialize)]
//!     pub struct RandomNumber {
//!         min: u32, // minimum value of the random number
//!         max: u32, // maximum value of the random number
//!     }
//!
//!     fn get_random_number(params: RandomNumber) -> Value {
//!         let random = 4;
//!         random.into()
//!     }
//!
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let messages = vec![ChatMessage {
//!         role: Role::User,
//!         content: "Can you get a random number between 1 and 6 please?".to_string(),
//!         ..Default::default()
//!     }];
//!
//!     let mut parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0613".to_string(),
//!         messages: messages.clone(),
//!         functions: Some(vec![Function {
//!             name: "get_random_number".to_string(),
//!             description: Some("Get a random number between two values".to_string()),
//!             parameters: json!({
//!                 "type": "object",
//!                 "properties": {
//!                     "min": {"type": "integer", "description": "Minimum value of the random number (inclusive)"},
//!                     "max": {"type": "integer", "description": "Maximum value of the random number (inclusive)"},
//!                 }
//!             }),
//!         }]),
//!         ..Default::default()
//!     };
//!
//!     let mut stream = client
//!         .chat()
//!         .create_stream(parameters.clone())
//!         .await
//!         .unwrap();
//!
//!     let mut function_call = FunctionCall::default();
//!     while let Some(response) = stream.next().await {
//!         match response {
//!             Ok(chat_response) => {
//!                 for choice in chat_response.choices {
//!                     if let Some(delta_function_call) = &choice.delta.function_call {
//!                         function_call.merge(delta_function_call);
//!                     } else if let Some(content) = choice.delta.content.as_ref() {
//!                         print!("{}", content);
//!                     }
//!
//!                     if choice.finish_reason == Some(FinishReason::FunctionCall)
//!                         && !function_call.is_empty()
//!                     {
//!                         if function_call.name == "get_random_number" {
//!                             let random_number_params =
//!                                 serde_json::from_str(&function_call.arguments).unwrap();
//!                             let random_number_result = get_random_number(random_number_params);
//!
//!                             parameters.messages.push(ChatMessage {
//!                                 role: Role::Function,
//!                                 content: serde_json::to_string(&random_number_result).unwrap(),
//!                                 name: Some("get_random_number".to_string()),
//!                                 ..Default::default()
//!                             });
//!
//!                             let mut stream = client
//!                                 .chat()
//!                                 .create_stream(parameters.clone())
//!                                 .await
//!                                 .unwrap();
//!
//!                             while let Some(response) = stream.next().await {
//!                                 match response {
//!                                     Ok(chat_response) => {
//!                                         chat_response.choices.iter().for_each(|choice| {
//!                                             if let Some(content) = choice.delta.content.as_ref() {
//!                                                 print!("{}", content);
//!                                             }
//!                                         })
//!                                     }
//!                                     Err(e) => eprintln!("{}", e),
//!                                 }
//!                             }
//!                         }
//!                     }
//!                 }
//!             }
//!             Err(e) => eprintln!("{}", e),
//!         }
//!     }
//! }
//! ```
//!
//! More information: [Function calling](https://platform.openai.com/docs/guides/function-calling)
//!
//! ## Create image
//!
//! > To download and save an image the feature `download` is required
//!
//! Creates an image given a prompt.
//!
//! **URL** `https://api.openai.com/v1/images/generations`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::image::{CreateImageParameters, ImageSize};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = CreateImageParameters {
//!         prompt: "A cute baby dog".to_string(),
//!         n: Some(1),
//!         size: Some(ImageSize::Size256X256),
//!         response_format: None,
//!         user: None,
//!     };
//!
//!     let result = client.images().create(parameters).await.unwrap();
//!
//!     // (optional) downloads and saves the image(s) to the folder called `images`
//!     let _paths = result.save("./images").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)
//!
//! ## Create image edit
//!
//! > To download and save an image the feature `download` is required
//!
//! Creates an edited or extended image given an original image and a prompt.
//!
//! **URL** `https://api.openai.com/v1/images/edits`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::image::{EditImageParameters, ImageSize};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = EditImageParameters {
//!         image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
//!         mask: Some("./images/image_edit_mask.png".to_string()), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_mask.png
//!         prompt: "A cute baby sea otter wearing a beret".to_string(),
//!         n: Some(1),
//!         size: Some(ImageSize::Size256X256),
//!         response_format: None,
//!         user: None,
//!     };
//!
//!     let result = client.images().edit(parameters).await.unwrap();
//!
//!     // (optional) downloads and saves the image(s) to the folder called `images`
//!     let _paths = result.save("./images").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/create-edit)
//!
//! ## Create image variation
//!
//! > To download and save an image the feature `download` is required
//!
//! Creates a variation of a given image.
//!
//! **URL** `https://api.openai.com/v1/images/variations`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::image::{CreateImageVariationParameters, ImageSize};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = CreateImageVariationParameters {
//!         image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
//!         n: Some(1),
//!         size: Some(ImageSize::Size256X256),
//!         response_format: None,
//!         user: None,
//!     };
//!
//!     let result = client.images().variation(parameters).await.unwrap();
//!
//!     // (optional) downloads and saves the image(s) to the folder called `images`
//!     let _paths = result.save("./images").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create image variation](https://platform.openai.com/docs/api-reference/images/create-variation)
//!
//! ## Create embedding
//!
//! Creates an embedding vector representing the input text.
//!
//! **URL** `https://api.openai.com/v1/embeddings`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::embedding::EmbeddingParameters;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = EmbeddingParameters {
//!         model: "text-embedding-ada-002".to_string(),
//!         input: "The food was delicious and the waiter...".to_string(),
//!         user: None,
//!     };
//!
//!     let result = client.embeddings().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create embedding](https://platform.openai.com/docs/api-reference/embeddings/create)
//!
//! ## Create transcription
//!
//! Transcribes audio into the input language.
//!
//! **URL** `https://api.openai.com/v1/audio/transcriptions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::audio::{AudioTranscriptionParameters, AudioTranscriptOutputFormat};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = AudioTranscriptionParameters {
//!         file: "./audio/micro-machines.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/micro-machines.mp3
//!         model: "whisper-1".to_string(),
//!         prompt: None,
//!         response_format: Some(AudioTranscriptOutputFormat::Srt),
//!         temperature: None,
//!         language: None,
//!     };
//!
//!     let result = client.audio().create_transcription(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create transcription](https://platform.openai.com/docs/api-reference/audio/create)
//!
//! ## Create translation
//!
//! Translates audio into English.
//!
//! **URL** `https://api.openai.com/v1/audio/translations`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::audio::{AudioTranscriptOutputFormat, AudioTranslationParameters};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = AudioTranslationParameters {
//!         file: "./audio/multilingual.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/multilingual.mp3
//!         model: "whisper-1".to_string(),
//!         prompt: None,
//!         response_format: Some(AudioTranscriptOutputFormat::Srt),
//!         temperature: None,
//!     };
//!
//!     let result = client.audio().create_translation(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create translation](https://platform.openai.com/docs/api-reference/audio/create)
//!
//! ## List files
//!
//! Returns a list of files that belong to the user's organization.
//!
//! **URL** `https://api.openai.com/v1/files`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.files().list().await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [List files](https://platform.openai.com/docs/api-reference/files/list)
//!
//! ## Upload file
//!
//! Upload a file that contains document(s) to be used across various endpoints/features.
//!
//! **URL** `https://api.openai.com/v1/files`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::file::UploadFileParameters;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = UploadFileParameters {
//!         file: "./files/SentimentAnalysisSample.jsonl".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/SentimentAnalysisSample.jsonl
//!         purpose: "fine-tune".to_string(),
//!     };
//!
//!     let result = client.files().upload(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Upload file](https://platform.openai.com/docs/api-reference/files/upload)
//!
//! ## Delete file
//!
//! Delete a file.
//!
//! **URL** `https://api.openai.com/v1/files/{file_id}`
//!
//! **Method** `DELETE`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.files().delete("file-XXX").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Delete file](https://platform.openai.com/docs/api-reference/files/delete)
//!
//! ## Retrieve file
//!
//! Returns information about a specific file.
//!
//! **URL** `https://api.openai.com/v1/files/{file_id}`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.files().retrieve("file-XXX").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Retrieve file](https://platform.openai.com/docs/api-reference/files/retrieve)
//!
//! ## Retrieve file content
//!
//! Returns the contents of the specified file.
//!
//! **URL** `https://api.openai.com/v1/files/{file_id}/content`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.files().retrieve_content("file-XXX").await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Retrieve file content](https://platform.openai.com/docs/api-reference/files/retrieve-content)
//!
//! ### Fine-tuning
//!
//! Manage fine-tuning jobs to tailor a model to your specific training data.
//!
//! See the `examples` directory for examples and implementation details.
//!
//! - [X] Create fine-tuning job
//! - [X] List fine-tuning jobs
//! - [X] Retrieve fine-tuning job
//! - [X] Cancel fine-tuning job
//! - [X] List fine-tune events
//!
//! More information: [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
//!
//! ## Create moderation
//!
//! Classifies if text violates OpenAI's Content Policy.
//!
//! **URL** `https://api.openai.com/v1/files/{file_id}/content`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::moderation::ModerationParameters;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = ModerationParameters {
//!         input: "I want to kill them.".to_string(),
//!         model: "text-moderation-latest".to_string(),
//!     };
//!
//!     let result = client.moderations().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create moderation](https://platform.openai.com/docs/api-reference/moderations)
//!
//! ## Set API key
//!
//! Add the OpenAI API key to your environment variables.
//!
//! ```sh
//! # Windows PowerShell
//! $Env:OPENAI_API_KEY='sk-...'
//!
//! # Windows cmd
//! set OPENAI_API_KEY=sk-...
//!
//! # Linux/macOS
//! export OPENAI_API_KEY='sk-...'
//! ```
//!
//! ## Add proxy
//!
//! This crate uses `reqwest` as HTTP Client. Reqwest has proxies enabled by default. You can set the proxy via the system environment variable or by overriding the default client.
//!
//! ### Example: set system environment variable
//!
//! You can set the proxy in the system environment variables ([https://docs.rs/reqwest/latest/reqwest/#proxies](https://docs.rs/reqwest/latest/reqwest/#proxies)).
//!
//! ```sh
//! export https_proxy=socks5://127.0.0.1:1086
//! ```
//!
//! ### Example: overriding the default client
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! let http_client = reqwest::Client::builder()
//!     .proxy(reqwest::Proxy::https("socks5://127.0.0.1:1086")?)
//!     .build()?;
//!
//! let client = Client {
//!     http_client,
//!     base_url: "https://api.openai.com/v1".to_string(),
//!     api_key: "YOUR API KEY".to_string(),
//! };
//! ```
//!
//! ## Use model names
//!
//! [https://platform.openai.com/docs/models/overview](https://platform.openai.com/docs/models/overview)
//!
//! ```rust
//! use openai_dive::v1::models::OpenAIModel;
//!
//! assert_eq!(OpenAIModel::Gpt4.to_string(), "gpt-4");
//! assert_eq!(OpenAIModel::Gpt4_0613.to_string(), "gpt-4-0613");
//! assert_eq!(OpenAIModel::Gpt4_32K.to_string(), "gpt-4-32k");
//! assert_eq!(OpenAIModel::Gpt4_32K0613.to_string(), "gpt-4-32k-0613");
//! assert_eq!(OpenAIModel::Gpt3_5Turbo.to_string(), "gpt-3.5-turbo");
//! assert_eq!(OpenAIModel::TextEmbeddingAda002.to_string(), "text-embedding-ada-002");
//! assert_eq!(OpenAIModel::Whisper1.to_string(), "whisper-1");
//! assert_eq!(OpenAIModel::TextModerationStable.to_string(), "text-moderation-stable");
//! assert_eq!(OpenAIModel::TextModerationLatest.to_string(), "text-moderation-latest");
//!

pub mod v1;
