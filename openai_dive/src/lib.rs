//! # OpenAI Dive
//!
//! OpenAI Dive is an unofficial async Rust library that allows you to interact with the OpenAI API.
//!
//! Sign up for an account on [https://platform.openai.com/overview](https://platform.openai.com/overview) to get your API key.
//!
//! ```ini
//! [dependencies]
//! openai_dive = "0.5"
//! ```
//!
//! ## Get started
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//! let client = Client::new(api_key); // or Client::new_from_env()
//!
//! let result = client
//!     .models()
//!     .list()
//!     .await?;
//! ```
//!
//! - [Set API key](#set-api-key)
//! - [Set organization/project id](#set-organizationproject-id)
//! - [Add proxy](#add-proxy)
//! - [Available models](#available-models)
//!
//! ## Endpoints
//!
//! - [Chat](#chat)
//!   - [Create chat completion](#create-chat-completion)
//!   - [Create chat completion with image](#create-chat-completion-with-image)
//!   - [Function calling](#function-calling)
//!   - [Structured outputs](#structured-outputs)
//! - [Images](#images)
//!   - [Create image](#create-image)
//!   - [Create image edit](#create-image-edit)
//!   - [Create image variation](#create-image-variation)
//! - [Audio](#audio)
//!   - [Create speech](#create-speech)
//!   - [Create transcription](#create-transcription)
//!   - [Create translation](#create-translation)
//! - [Models](#models)
//! - [Files](#files)
//! - [Embeddings](#embeddings)
//! - [Moderation](#moderation)
//! - [Uploads](#uploads)
//! - [Fine-tuning](#fine-tuning)
//! - [Batches](#batches)
//! - [Assistants](#assistants)
//! - [Administration](#administration)
//!
//! ## Chat
//!
//! Given a list of messages comprising a conversation, the model will return a response.
//!
//! ### Create chat completion
//!
//! Creates a model response for the given chat conversation.
//!
//! ```rust
//! let parameters = ChatCompletionParametersBuilder::default()
//!     .model(Gpt4Engine::Gpt4O.to_string())
//!     .messages(vec![
//!         ChatMessageBuilder::default()
//!             .role(Role::User)
//!             .content(ChatMessageContent::Text("Hello!".to_string()))
//!             .build()?,
//!         ChatMessageBuilder::default()
//!             .role(Role::User)
//!             .content(ChatMessageContent::Text(
//!                 "What is the capital of Vietnam?".to_string(),
//!             ))
//!             .build()?,
//!     ])
//!     .response_format(ChatCompletionResponseFormat::Text)
//!     .build()?;
//!
//! let result = client
//!     .chat()
//!     .create(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ### Create chat completion with image
//!
//! Creates a model response for the given chat conversation.
//!
//! ```rust
//! let parameters = ChatCompletionParametersBuilder::default()
//!     .model(Gpt4Engine::Gpt4O.to_string())
//!     .messages(vec![
//!         ChatMessageBuilder::default()
//!             .role(Role::User)
//!             .content(ChatMessageContent::Text(
//!                 "What is in this image?".to_string(),
//!             ))
//!             .build()?,
//!         ChatMessageBuilder::default()
//!             .role(Role::User)
//!             .content(ChatMessageContent::ImageUrl(vec![ImageUrl {
//!                 r#type: "image_url".to_string(),
//!                 text: None,
//!                 image_url: ImageUrlType {
//!                     url: "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640"
//!                         .to_string(),
//!                     detail: None,
//!                 },
//!             }]))
//!             .build()?,
//!     ])
//!     .max_tokens(50u32)
//!     .build()?;
//!
//! let result = client
//!     .chat()
//!     .create(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ### Function calling
//!
//! In an API call, you can describe functions and have the model intelligently choose to output a JSON object containing arguments to call one or many functions. The Chat Completions API does not call the function; instead, the model generates JSON that you can use to call the function in your code.
//!
//! ```rust
//! let messages = vec![ChatMessageBuilder::default()
//!     .content(ChatMessageContent::Text(
//!         "Give me a random number between 100 and no more than 150?".to_string(),
//!     ))
//!     .build()?];
//!
//! let parameters = ChatCompletionParametersBuilder::default()
//!     .model(Gpt4Engine::Gpt4O.to_string())
//!     .messages(messages)
//!     .tools(vec![ChatCompletionTool {
//!         r#type: ChatCompletionToolType::Function,
//!         function: ChatCompletionFunction {
//!             name: "get_random_number".to_string(),
//!             description: Some("Get a random number between two values".to_string()),
//!             parameters: json!({
//!                 "type": "object",
//!                 "properties": {
//!                     "min": {"type": "integer", "description": "Minimum value of the random number."},
//!                     "max": {"type": "integer", "description": "Maximum value of the random number."},
//!                 },
//!                 "required": ["min", "max"],
//!             }),
//!         },
//!     }])
//!     .build()?;
//!
//! let result = client
//!     .chat()
//!     .create(parameters)
//!     .await?;
//!
//! let message = result.choices[0].message.clone();
//!
//! if let Some(tool_calls) = message.tool_calls {
//!     for tool_call in tool_calls {
//!         let name = tool_call.function.name;
//!         let arguments = tool_call.function.arguments;
//!
//!         if name == "get_random_number" {
//!             let random_numbers: RandomNumber = serde_json::from_str(&arguments)?;
//!
//!             println!("Min: {:?}", &random_numbers.min);
//!             println!("Max: {:?}", &random_numbers.max);
//!
//!             let random_number_result = get_random_number(random_numbers);
//!
//!             println!(
//!                 "Random number between those numbers: {:?}",
//!                 random_number_result.clone()
//!             );
//!         }
//!     }
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct RandomNumber {
//!     min: u32,
//!     max: u32,
//! }
//!
//! fn get_random_number(params: RandomNumber) -> Value {
//!     let random_number = rand::thread_rng().gen_range(params.min..params.max);
//!
//!     random_number.into()
//! }
//! ```
//!
//! More information: [Function calling](https://platform.openai.com/docs/guides/function-calling)
//!
//! ### Structured outputs
//!
//! Structured Outputs is a feature that guarantees the model will always generate responses that adhere to your supplied JSON Schema, so you don't need to worry about the model omitting a required key, or hallucinating an invalid enum value.
//!
//! ```rust
//! let parameters = ChatCompletionParametersBuilder::default()
//!     .model("gpt-4o-2024-08-06")
//!     .messages(vec![
//!         ChatMessageBuilder::default()
//!             .role(Role::System)
//!             .content(ChatMessageContent::Text("You are a helpful math tutor. Guide the user through the solution step by step.".to_string()))
//!             .build()?,
//!         ChatMessageBuilder::default()
//!             .role(Role::User)
//!             .content(ChatMessageContent::Text(
//!                 "how can I solve 8x + 7 = -23".to_string(),
//!             ))
//!             .build()?,
//!     ])
//!     .response_format(ChatCompletionResponseFormat::JsonSchema(JsonSchemaBuilder::default()
//!         .name("math_reasoning")
//!         .schema(serde_json::json!({
//!             "type": "object",
//!             "properties": {
//!                 "steps": {
//!                     "type": "array",
//!                     "items": {
//!                         "type": "object",
//!                         "properties": {
//!                             "explanation": { "type": "string" },
//!                             "output": { "type": "string" }
//!                         },
//!                         "required": ["explanation", "output"],
//!                         "additionalProperties": false
//!                     }
//!                 },
//!                 "final_answer": { "type": "string" }
//!             },
//!             "required": ["steps", "final_answer"],
//!             "additionalProperties": false
//!         }))
//!         .strict(true)
//!         .build()?
//!     ))
//!     .build()?;
//!
//! let result = client.chat().create(parameters).await?;
//!
//! println!("{:#?}", result);
//! ```
//!
//! More information: [Structured outputs](https://platform.openai.com/docs/guides/structured-outputs)
//!
//! ## Images
//!
//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! ### Create image
//!
//! Creates an image given a prompt.
//!
//! ```rust
//! let parameters = CreateImageParametersBuilder::default()
//!     .prompt("A cute dog in the park")
//!     .model(DallEEngine::DallE3.to_string())
//!     .n(1u32)
//!     .quality(ImageQuality::Standard)
//!     .response_format(ResponseFormat::Url)
//!     .size(ImageSize::Size1024X1024)
//!     .style(ImageStyle::Natural)
//!     .build()?;
//!
//! let result = client
//!     .images()
//!     .create(parameters)
//!     .await?;
//!
//! let paths = result
//!     .save("./images")
//!     .await?;
//! ```
//!
//! More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)
//!
//! ### Create image edit
//!
//! Creates an edited or extended image given an original image and a prompt.
//!
//! ```rust
//! let parameters = EditImageParametersBuilder::default()
//!     .image(FileUpload::File(
//!         "./images/image_edit_original.png".to_string(),
//!     ))
//!     .prompt("A cute baby sea otter")
//!     .mask(FileUpload::File("./images/image_edit_mask.png".to_string()))
//!     .n(1u32)
//!     .size(ImageSize::Size512X512)
//!     .build()?;
//!
//! let result = client
//!     .images()
//!     .edit(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/createEdit)
//!
//! ### Create image variation
//!
//! Creates a variation of a given image.
//!
//! ```rust
//! let parameters = CreateImageVariationParametersBuilder::default()
//!     .image(FileUpload::File(
//!         "./images/image_edit_original.png".to_string(),
//!     ))
//!     .n(1u32)
//!     .size(ImageSize::Size256X256)
//!     .build()?;
//!
//! let result = client
//!     .images()
//!     .variation(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create image variation](https://platform.openai.com/docs/api-reference/images/createVariation)
//!
//! ## Audio
//!
//! Learn how to turn audio into text or text into audio.
//!
//! ### Create speech
//!
//! Generates audio from the input text.
//!
//! ```rust
//! let parameters = AudioSpeechParametersBuilder::default()
//!     .model(TTSEngine::Tts1.to_string())
//!     .input("Hallo, this is a test from OpenAI Dive.")
//!     .voice(AudioVoice::Alloy)
//!     .response_format(AudioSpeechResponseFormat::Mp3)
//!     .build()?;
//!
//! let response = client
//!     .audio()
//!     .create_speech(parameters)
//!     .await?;
//!
//! response
//!     .save("files/example.mp3")
//!     .await?;
//! ```
//!
//! More information: [Create speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
//!
//! ### Create transcription
//!
//! Transcribes audio into the input language.
//!
//! ```rust
//! let parameters = AudioTranscriptionParametersBuilder::default()
//!     .file(FileUpload::File("./audio/micro-machines.mp3".to_string()))
//!     .model(WhisperEngine::Whisper1.to_string())
//!     .response_format(AudioOutputFormat::VerboseJson)
//!     .build()?;
//!
//! let result = client
//!     .audio()
//!     .create_transcription(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create transcription](https://platform.openai.com/docs/api-reference/audio/createTranscription)
//!
//! ### Create translation
//!
//! Translates audio into English.
//!
//! ```rust
//! let parameters = AudioTranslationParametersBuilder::default()
//!     .file(FileUpload::File("./audio/multilingual.mp3".to_string()))
//!     .model(WhisperEngine::Whisper1.to_string())
//!     .response_format(AudioOutputFormat::Srt)
//!     .build()?;
//!
//! let result = client
//!     .audio()
//!     .create_translation(parameters)
//!     .await?;
//! ```
//!
//! More information: [Create translation](https://platform.openai.com/docs/api-reference/audio/createTranslation)
//!
//! ## Models
//!
//! List and describe the various models available in the API.
//!
//! For more information see the examples in the [examples/models](https://github.com/tjardoo/openai-client/tree/master/examples/models) directory.
//!
//! - List models
//! - Retrieve model
//! - Delete fine-tune model
//!
//! More information [Models](https://platform.openai.com/docs/api-reference/models)
//!
//! ## Files
//!
//! Files are used to upload documents that can be used with features like Assistants, Fine-tuning, and Batch API.
//!
//! For more information see the examples in the [examples/files](https://github.com/tjardoo/openai-client/tree/master/examples/files) directory.
//!
//! - List files
//! - Upload file
//! - Delete file
//! - Retrieve file
//! - Retrieve file content
//!
//! More information [Files](https://platform.openai.com/docs/api-reference/files)
//!
//! ## Embeddings
//!
//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! For more information see the examples in the [examples/embeddings](https://github.com/tjardoo/openai-client/tree/master/examples/embeddings) directory.
//!
//! - Create embeddings
//!
//! More information: [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
//!
//! ## Moderation
//!
//! Given some input text, outputs if the model classifies it as potentially harmful across several categories.
//!
//! For more information see the examples in the [examples/moderation](https://github.com/tjardoo/openai-client/tree/master/examples/moderation) directory.
//!
//! - Create moderation
//!
//! More information [Moderation](https://platform.openai.com/docs/api-reference/moderations)
//!
//! ### Uploads
//!
//! Creates an intermediate Upload object that you can add Parts to. Currently, an Upload can accept at most 8 GB in total and expires after an hour after you create it.
//!
//! Once you complete the Upload, we will create a File object that contains all the parts you uploaded. This File is usable in the rest of our platform as a regular File object.
//!
//! For more information see the examples in the [examples/uploads](https://github.com/tjardoo/openai-client/tree/master/examples/uploads) directory.
//!
//! - Create upload
//! - Add upload part
//! - Complete upload
//! - Cancel upload
//!
//! More information [Uploads](https://platform.openai.com/docs/api-reference/uploads)
//!
//! ### Fine-tuning
//!
//! Manage fine-tuning jobs to tailor a model to your specific training data.
//!
//! For more information see the examples in the [examples/fine_tuning](https://github.com/tjardoo/openai-client/tree/master/examples/fine_tuning) directory.
//!
//! - Create fine-tuning job
//! - List fine-tuning jobs
//! - Retrieve fine-tuning job
//! - Cancel fine-tuning job
//! - List fine-tuning events
//! - List fine-tuning checkpoints
//!
//! More information [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
//!
//! ### Batches
//!
//! Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
//!
//! For more information see the examples in the [examples/batches](https://github.com/tjardoo/openai-client/tree/master/examples/batches) directory.
//!
//! - Create batch
//! - List batches
//! - Retrieve batch
//! - Cancel batch
//!
//! More information [Batch](https://platform.openai.com/docs/api-reference/batch)
//!
//! ## Assistants
//!
//! Build assistants that can call models and use tools to perform tasks.
//!
//! For more information see the examples in the [examples/assistants](https://github.com/tjardoo/openai-client/tree/master/examples/assistants) directory.
//!
//! - Assistants
//! - Threads
//! - Messages
//! - Runs
//! - Run Steps
//!
//! More information [Assistants](https://platform.openai.com/docs/api-reference/assistants)
//!
//! ### Administration
//!
//! Programmatically manage your organization.
//!
//! For more information see the examples in the [examples/administration](https://github.com/tjardoo/openai-client/tree/master/examples/administration) directory.
//!
//! - Users
//! - Invites
//! - Projects
//! - Project Users
//! - Project Service Accounts
//! - Project API Keys
//!
//! ## Configuration
//!
//! ### Set API key
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
//! ### Set organization/project ID
//!
//! You can create multiple organizations and projects in the OpenAI platform. This allows you to group files, fine-tuned models and other resources.
//!
//! You can set the organization ID and/or project ID on the client via the `set_organization` and `set_project` methods. If you don't set the organization and/or project ID, the client will use the default organization and default project.
//!
//! ```rust
//! let mut client = Client::new(api_key);
//!
//! client
//!     .set_organization("org-XXX")
//!     .set_project("proj_XXX");
//! ```
//!
//! ### Add proxy
//!
//! This crate uses `reqwest` as HTTP Client. Reqwest has proxies enabled by default. You can set the proxy via the system environment variable or by overriding the default client.
//!
//! #### Example: set system environment variable
//!
//! You can set the proxy in the system environment variables ([https://docs.rs/reqwest/latest/reqwest/#proxies](https://docs.rs/reqwest/latest/reqwest/#proxies)).
//!
//! ```sh
//! export HTTPS_PROXY=socks5://127.0.0.1:1086
//! ```
//!
//! #### Example: overriding the default client
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! let http_client = reqwest::Client::builder()
//!     .proxy(reqwest::Proxy::https("socks5://127.0.0.1:1086")?)
//!     .build()?;
//!
//! let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
//!
//! let client = Client {
//!     http_client,
//!     base_url: "https://api.openai.com/v1".to_string(),
//!     api_key,
//!     headers: None,
//!     organization: None,
//!     project: None,
//! };
//! ```
//!
//! ### Available Models
//!
//! You can use these predefined constants to set the model in the parameters or use any string representation (ie. for your custom models).
//!
//! - Gpt4Engine
//!   - Gpt4O `gpt-4o` (alias)
//!   - Gpt4OMini `gpt-4o-mini` (alias)
//!   - Gpt4 `gpt-4` (alias)
//!   - Gpt4Turbo `gpt-4-turbo` (alias)
//!   - Gpt4TurboPreview `gpt-4-turbo-preview` (alias)
//! - Gpt35Engine
//!   - Gpt35Turbo `gpt-3.5-turbo` (alias)
//!   - Gpt35Turbo1106 `gpt-3.5-turbo-1106`
//! - DallEEngine
//!   - DallE3 `dall-e-2`
//!   - DallE2 `dall-e-3`
//! - TTSEngine
//!   - Tts1 `tts-1`
//!   - Tts1HD `tts-1-hd`
//! - WhisperEngine
//!   - Whisper1 `whisper-1`
//! - EmbeddingsEngine
//!   - TextEmbedding3Small `text-embedding-3-small`
//!   - TextEmbedding3Large `text-embedding-3-large`
//!   - TextEmbeddingAda002 `text-embedding-ada-002`
//! - ModerationsEngine
//!   - TextModerationLatest `text-moderation-latest` (alias)
//!   - TextModerationStable `text-moderation-stable` (alias)
//!   - TextModeration007 `text-moderation-007`
//!
//! More information: [Models](https://platform.openai.com/docs/models)

pub mod v1;
