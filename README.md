# OpenAI Dive

[![crates.io](https://img.shields.io/crates/v/openai_dive.svg?style=flat-square)](https://crates.io/crates/openai_dive)
![cargo build](https://img.shields.io/github/actions/workflow/status/tjardoo/openai-client/cargo-build.yml?style=flat-square)
[![docs.rs](https://img.shields.io/docsrs/openai_dive?style=flat-square)](https://docs.rs/openai_dive)
[![crates.io](https://img.shields.io/crates/d/openai_dive.svg?style=flat-square)](https://crates.io/crates/openai_dive)

OpenAI Dive is an unofficial async Rust library that allows you to interact with the OpenAI API.

Sign up for an account on [https://platform.openai.com/overview](https://platform.openai.com/overview) to get your API key.

```ini
[dependencies]
openai_dive = "0.4"
```

More information: [set API key](#set-api-key), [add proxy](#add-proxy), [rate limit headers](#rate-limit-headers), [use model names](#use-model-names)

## Endpoints

- Models
  - [List models](#list-models)
  - [Retrieve model](#retrieve-model)
  - [Delete fine-tune model](#delete-fine-tune-model)
- Chat
  - [Create chat completion](#create-chat-completion)
  - [Create chat completion with image](#create-chat-completion-with-image)
  - [Function calling](#function-calling)
- Images
  - [Create image](#create-image)
  - [Create image edit](#create-image-edit)
  - [Create image variation](#create-image-variation)
- Audio
  - [Create speech](#create-speech)
  - [Create transcription](#create-transcription)
  - [Create translation](#create-translation)
- Embeddings
  - [Create embeddings](#create-embeddings)
- Files
  - [List files](#list-files)
  - [Upload file](#upload-file)
  - [Delete file](#delete-file)
  - [Retrieve file](#retrieve-file)
  - [Retrieve file content](#retrieve-file-content)
- Fine tuning
  - [Create fine tuning job](#create-fine-tuning-job)
  - [List fine tuning jobs](#list-fine-tuning-jobs)
  - [Retrieve fine tuning job](#retrieve-fine-tuning-job)
  - [Cancel fine tuning](#cancel-fine-tuning)
  - [List fine tuning events](#list-fine-tuning-events)
- Moderation
  - [Create moderation](#create-moderation)
- Assistants
  - [Assistants (beta)](#assistants-beta)

## Models

List and describe the various models available in the API.

### List models

Lists the currently available models, and provides basic information about each one such as the owner and availability.

```rust
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().list().await.unwrap();

    println!("{:#?}", result);
}
```

More information: [List models](https://platform.openai.com/docs/api-reference/models/list)

### Retrieve model

Retrieves a model instance, providing basic information about the model such as the owner and permissioning.

```rust
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().get("gpt-3.5-turbo-16k-0613").await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Retrieve model](https://platform.openai.com/docs/api-reference/models/retrieve)

### Delete fine-tune model

Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.

```rust
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().delete("my-custom-model").await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Delete fine-tune model](https://platform.openai.com/docs/api-reference/models/delete)

## Chat

Given a list of messages comprising a conversation, the model will return a response.

### Create chat completion

Creates a model response for the given chat conversation.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/chat/create_chat_completion_stream](https://github.com/tjardoo/openai-client/tree/master/examples/chat/create_chat_completion_stream) example.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt41106Preview.to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("Hello!".to_string()),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("What is the capital of Vietnam?".to_string()),
                ..Default::default()
            },
        ],
        max_tokens: Some(12),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Create chat completion with image

Creates a model response for the given chat conversation.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt4VisionPreview.to_string(),
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
                        url: "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640".to_string(),
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
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Function calling

In an API call, you can describe functions and have the model intelligently choose to output a JSON object containing arguments to call one or many functions. The Chat Completions API does not call the function; instead, the model generates JSON that you can use to call the function in your code.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/chat/function_calling_stream](https://github.com/tjardoo/openai-client/tree/master/examples/chat/function_calling_stream) example.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParameters, ChatCompletionTool, ChatCompletionToolType, ChatMessage,
    ChatMessageContent,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let messages = vec![ChatMessage {
        content: ChatMessageContent::Text("Give me a random number between 100 and no more than 150?".to_string()),
        ..Default::default()
    }];

    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt41106Preview.to_string(),
        messages: messages.clone(),
        tools: Some(vec![ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: "get_random_number".to_string(),
                description: Some("Get a random number between two values".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "min": {"type": "integer", "description": "Minimum value of the random number."},
                        "max": {"type": "integer", "description": "Maximum value of the random number."},
                    },
                    "required": ["min", "max"],
                }),
            },
        }]),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    let message = result.choices[0].message.clone();

    if let Some(tool_calls) = message.tool_calls {
        for tool_call in tool_calls {
            let name = tool_call.function.name;
            let arguments = tool_call.function.arguments;

            if name == "get_random_number" {
                let random_numbers: RandomNumber = serde_json::from_str(&arguments).unwrap();

                println!("Min: {:?}", &random_numbers.min);
                println!("Max: {:?}", &random_numbers.max);

                let random_number_result = get_random_number(random_numbers);

                println!("Random number between those numbers: {:?}", random_number_result.clone());
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RandomNumber {
    min: u32,
    max: u32,
}

fn get_random_number(params: RandomNumber) -> Value {
    let random_number = rand::thread_rng().gen_range(params.min..params.max);

    random_number.into()
}
```

More information: [Function calling](https://platform.openai.com/docs/guides/function-calling)

## Images

Given a prompt and/or an input image, the model will generate a new image.

### Create image

Creates an image given a prompt.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageParameters, ImageSize, ResponseFormat};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageParameters {
        prompt: "A cute baby dog".to_string(),
        model: None,
        n: Some(1),
        quality: None,
        response_format: Some(ResponseFormat::Url),
        size: Some(ImageSize::Size256X256),
        style: None,
        user: None,
    };

    let result = client.images().create(parameters).await.unwrap();

    let paths = result.save("./images").await.unwrap();

    println!("{:?}", paths);

    println!("{:#?}", result);
}
```

More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)

### Create image edit

Creates an edited or extended image given an original image and a prompt.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParameters, ImageSize};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditImageParameters {
        image: "./images/image_edit_original.png".to_string(),
        prompt: "A cute baby sea otter".to_string(),
        mask: Some("./images/image_edit_mask.png".to_string()),
        model: None,
        n: Some(1),
        size: Some(ImageSize::Size256X256),
        response_format: None,
        user: None,
    };

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/createEdit)

### Create image variation

Creates a variation of a given image.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageVariationParameters, ImageSize};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageVariationParameters {
        image: "./images/image_edit_original.png".to_string(),
        model: None,
        n: Some(1),
        response_format: None,
        size: Some(ImageSize::Size256X256),
        user: None,
    };

    let result = client.images().variation(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Create image variation](https://platform.openai.com/docs/api-reference/images/createVariation)

## Audio

Learn how to turn audio into text or text into audio.

### Create speech

Generates audio from the input text.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/audio/create_speech_stream](https://github.com/tjardoo/openai-client/tree/master/examples/audio/create_speech_stream) example.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{
    AudioSpeechParameters, AudioSpeechResponseFormat, AudioVoice,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioSpeechParameters {
        model: "tts-1".to_string(),
        input: "Hallo, this is a test from OpenAI Dive.".to_string(),
        voice: AudioVoice::Alloy,
        response_format: Some(AudioSpeechResponseFormat::Mp3),
        speed: Some(1.0),
    };

    let response = client.audio().create_speech(parameters).await.unwrap();

    response.save("files/example.mp3").await.unwrap();
}
```

More information: [Create speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)

### Create transcription

Transcribes audio into the input language.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranscriptionParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranscriptionParameters {
        file: "./audio/micro-machines.mp3".to_string(),
        model: "whisper-1".to_string(),
        language: None,
        prompt: None,
        response_format: Some(AudioOutputFormat::Text),
        temperature: None,
    };

    let result = client
        .audio()
        .create_transcription(parameters)
        .await
        .unwrap();

    println!("{:#?}", result);
}
```

More information: [Create transcription](https://platform.openai.com/docs/api-reference/audio/createTranscription)

### Create translation

Translates audio into English.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranslationParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranslationParameters {
        file: "./audio/multilingual.mp3".to_string(),
        model: "whisper-1".to_string(),
        prompt: None,
        response_format: Some(AudioOutputFormat::Srt),
        temperature: None,
    };

    let result = client.audio().create_translation(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Create translation](https://platform.openai.com/docs/api-reference/audio/createTranslation)

## Embeddings

Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.

### Create embeddings

Creates an embedding vector representing the input text.

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::embedding::{EmbeddingInput, EmbeddingParameters};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParameters {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("The food was delicious and the waiter...".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [Create embeddings](https://platform.openai.com/docs/api-reference/embeddings/create)

## Files

Files are used to upload documents that can be used with features like Assistants and Fine-tuning.

### List files

Returns a list of files that belong to the user's organization.

```rust
use openai_dive::v1::{
    api::Client,
    resources::file::{FilePurpose, ListFilesParameters},
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let query = ListFilesParameters {
        purpose: Some(FilePurpose::FineTune),
    };

    let result = client.files().list(Some(query)).await.unwrap();

    println!("{:#?}", result);
}
```

More information: [List files](https://platform.openai.com/docs/api-reference/files/list)

### Upload file

Upload a file that can be used across various endpoints.

```rust
use openai_dive::v1::{
    api::Client,
    resources::file::{FilePurpose, UploadFileParameters},
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = UploadFileParameters {
        file: "./files/FineTuningJobSample2.jsonl".to_string(),
        purpose: FilePurpose::FineTune,
    };

    let result = client.files().upload(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Upload file](https://platform.openai.com/docs/api-reference/files/create)

### Delete file

Delete a file.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().delete(&file_id).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Delete file](https://platform.openai.com/docs/api-reference/files/delete)

### Retrieve file

Returns information about a specific file.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().retrieve(&file_id).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Retrieve file](https://platform.openai.com/docs/api-reference/files/retrieve)

### Retrieve file content

Returns the contents of the specified file.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let result = client.files().retrieve_content(&file_id).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Retrieve file content](https://platform.openai.com/docs/api-reference/files/retrieve-contents)

### Fine tuning

Manage fine-tuning jobs to tailor a model to your specific training data.

### Create fine tuning job

Creates a job that fine-tunes a specified model from a given dataset.

```rust
use dotenv::dotenv;
use openai_dive::v1::{api::Client, resources::fine_tuning::CreateFineTuningJobParameters};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let file_id = env::var("FILE_ID").expect("FILE_ID is not set in the .env file.");

    let parameters = CreateFineTuningJobParameters {
        model: "gpt-3.5-turbo-1106".to_string(),
        training_file: file_id,
        hyperparameters: None,
        suffix: None,
        validation_file: None,
    };

    let result = client.fine_tuning().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Create fine tuning job](https://platform.openai.com/docs/api-reference/fine-tuning/create)

### List fine tuning jobs

List your organization's fine-tuning jobs.

```rust
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.fine_tuning().list(None).await.unwrap();

    println!("{:#?}", result);
}
```

More information [List fine tuning jobs](https://platform.openai.com/docs/api-reference/fine-tuning/list)

### Retrieve fine tuning job

Get info about a fine-tuning job.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let fine_tuning_job_id =
        env::var("FINE_TUNING_JOB_ID").expect("FINE_TUNING_JOB_ID is not set in the .env file.");

    let result = client
        .fine_tuning()
        .retrieve(&fine_tuning_job_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}
```

More information [Retrieve fine tuning jobs](https://platform.openai.com/docs/api-reference/fine-tuning/retrieve)

### Cancel fine tuning

Immediately cancel a fine-tune job.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let fine_tuning_job_id =
        env::var("FINE_TUNING_JOB_ID").expect("FINE_TUNING_JOB_ID is not set in the .env file.");

    let result = client
        .fine_tuning()
        .cancel(&fine_tuning_job_id)
        .await
        .unwrap();

    println!("{:#?}", result);
}
```

More information [Cancel fine tuning](https://platform.openai.com/docs/api-reference/fine-tuning/cancel)

### List fine tuning events

Get status updates for a fine-tuning job.

```rust
use dotenv::dotenv;
use openai_dive::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let fine_tuning_job_id =
        env::var("FINE_TUNING_JOB_ID").expect("FINE_TUNING_JOB_ID is not set in the .env file.");

    let result = client
        .fine_tuning()
        .list_job_events(&fine_tuning_job_id, None)
        .await
        .unwrap();

    println!("{:#?}", result);
}
```

More information [List fine tuning events](https://platform.openai.com/docs/api-reference/fine-tuning/list-events)

## Moderation

Given a input text, outputs if the model classifies it as violating OpenAI's content policy.

### Create moderation

Classifies if text violates OpenAI's Content Policy

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::moderation::ModerationParameters;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ModerationParameters {
        input: "I want to kill them.".to_string(),
        model: "text-moderation-latest".to_string(),
    };

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:#?}", result);
}
```

More information [Create moderation](https://platform.openai.com/docs/api-reference/moderations/create)

## Assistants (beta)

Build assistants that can call models and use tools to perform tasks.

For more information see the examples in the [examples/assistants](https://github.com/tjardoo/openai-client/tree/master/examples/assistants) directory.

- Assistants
- Files
- Threads
- Messages
- Runs

More information [Assistants](https://platform.openai.com/docs/api-reference/assistants)

## General

### Set API key

Add the OpenAI API key to your environment variables.

```sh
# Windows PowerShell
$Env:OPENAI_API_KEY='sk-...'

# Windows cmd
set OPENAI_API_KEY=sk-...

# Linux/macOS
export OPENAI_API_KEY='sk-...'
```

### Add proxy

This crate uses `reqwest` as HTTP Client. Reqwest has proxies enabled by default. You can set the proxy via the system environment variable or by overriding the default client.

#### Example: set system environment variable

You can set the proxy in the system environment variables ([https://docs.rs/reqwest/latest/reqwest/#proxies](https://docs.rs/reqwest/latest/reqwest/#proxies)).

```sh
export HTTPS_PROXY=socks5://127.0.0.1:1086
```

#### Example: overriding the default client

```rust
use openai_dive::v1::api::Client;

let http_client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::https("socks5://127.0.0.1:1086")?)
    .build()?;

let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

let client = Client {
    http_client,
    base_url: "https://api.openai.com/v1".to_string(),
    api_key,
};
```

## Rate limit headers

In addition to seeing your rate limit on your account page, you can also view important information about your rate limits such as the remaining requests, tokens, and other metadata in the headers of the HTTP response.

The following endpoints have rate limit headers support:

- [Create chat completion](#create-chat-completion)
- [Create embeddings](#create-embeddings)

You can access them by calling the `create_wrapped` method instead of the `create` method. The `create_wrapped` method returns a `Result<WrappedResponse<T>, Error>`.

```rust
use openai_dive::v1::api::Client;

let result = client.chat().create_wrapped(parameters).await.unwrap();

// the chat completion response
println!("{:#?}", result.data);

// the rate limit headers
println!("{:#?}", result.headers);
```

More information: [Rate limit headers](https://platform.openai.com/docs/guides/rate-limits/rate-limits-in-headers)

## Use model names

- Gpt4Engine
  - Gpt41106Preview `gpt-4-1106-preview`
  - Gpt4VisionPreview `gpt-4-vision-preview`
  - Gpt4 `gpt-4`
  - Gpt432K `gpt-4-32k`
  - Gpt40613 `gpt-4-0613`
  - Gpt432K0613 `gpt-4-32k-0613`
- Gpt35Engine
  - Gpt35Turbo1106 `gpt-3.5-turbo-1106`
  - Gpt35Turbo `gpt-3.5-turbo`
  - Gpt35Turbo16K `gpt-3.5-turbo-16k`
  - Gpt35TurboInstruct `gpt-3.5-turbo-instruct`
- DallEEngine
  - DallE3 `dall-e-2`
  - DallE2 `dall-e-3`
- TTSEngine
  - Tts1 `tts-1`
  - Tts1HD `tts-1-hd`
- WhisperEngine
  - Whisper1 `whisper-1`
- EmbeddingsEngine
  - TextEmbeddingAda002 `text-embedding-ada-002`
- ModerationsEngine
  - TextModerationLatest `text-moderation-latest`
  - TextModerationStable `text-moderation-stable`

More information: [Models](https://platform.openai.com/docs/models)
