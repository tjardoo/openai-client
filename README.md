# OpenAI Dive

[![crates.io](https://img.shields.io/crates/v/openai_dive.svg)](https://crates.io/crates/openai_dive)
[![docs.rs](https://docs.rs/openai_dive/badge.svg)](https://docs.rs/openai_dive)

OpenAI Dive is an unofficial async Rust library that allows you to interact with the OpenAI API.

Sign up for an account on [https://platform.openai.com/overview](https://platform.openai.com/overview) to get your API key.

```ini
[dependencies]
openai_dive = "0.2"
```

More information: [set API key](#set-api-key), [add proxy](#add-proxy)

## Endpoints

- Models
  - [List models](#list-models)
  - [Retrieve model](#retrieve-model)
- Completions
  - [Create completion](#create-completion)
  - [Create completion (stream)](#create-completion-stream)
- Chat
  - [Create chat completion](#create-chat-completion)
  - [Create chat completion (stream)](#create-chat-completion-stream)
- Edits
  - [Create edit](#create-edit)
- Images
  - [Create image](#create-image)
  - [Create image edit](#create-image-edit)
  - [Create image variation](#create-image-variation)
- Embeddings
  - [Create embedding](#create-embedding)
- Audio
  - [Create transcription](#create-transcription)
  - [Create translation](#create-translation)
- Files
  - [List files](#list-files)
  - [Upload file](#upload-file)
  - [Delete file](#delete-file)
  - [Retrieve file](#retrieve-file)
  - [Retrieve file content](#retrieve-file-content)
- Moderations
  - [Create moderation](#create-moderation)

### List models

Lists the currently available models, and provides basic information about each one such as the owner and availability.

**URL** `https://api.openai.com/v1/models`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().list().await.unwrap();

    println!("{:?}", result);
}
```

More information: [List models](https://platform.openai.com/docs/api-reference/models/list)

### Retrieve model

Retrieves a model instance, providing basic information about the model such as the owner and permissioning.

**URL** `https://api.openai.com/v1/models/{model}`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.models().get("text-davinci-003").await.unwrap();

    println!("{:?}", result);
}
```

More information: [Retrieve models](https://platform.openai.com/docs/api-reference/models/retrieve)

### Create completion

Creates a completion for the provided prompt and parameters.

**URL** `https://api.openai.com/v1/completions`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::completion::CompletionParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CompletionParameters {
        model: "text-davinci-003".to_string(),
        prompt: "Say this is a test".to_string(),
        suffix: None,
        max_tokens: 10,
        temperature: None,
    };

    let result = client.completions().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create completion](https://platform.openai.com/docs/api-reference/completions/create)

### Create completion (stream)

> Feature `stream` required

Creates a completion for the provided prompt and parameters.

**URL** `https://api.openai.com/v1/completions`

**Method** `POST`

```rust
use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::completion::CompletionParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CompletionParameters {
        model: "text-davinci-003".to_string(),
        prompt: "Create an outline for an essay about Nikola Tesla and his contributions to technology:".to_string(),
        suffix: None,
        max_tokens: 100,
        temperature: None,
    };

    let mut stream = client.completions().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(completion_response) => completion_response.choices.iter().for_each(|choice| {
                print!("{}", choice.text);
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
```

More information: [Create completion](https://platform.openai.com/docs/api-reference/completions/create)

### Create chat completion

Creates a completion for the chat message.

**URL** `https://api.openai.com/v1/chat/completions`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        max_tokens: 12,
        temperature: None,
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Create chat completion (stream)

> Feature `stream` required

Creates a completion for the chat message.

**URL** `https://api.openai.com/v1/chat/completions`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        max_tokens: 12,
        temperature: None,
    };

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if choice.delta.role.is_some() {
                    println!("role: {}", choice.delta.role.as_ref().unwrap());
                } else if choice.delta.content.is_some() {
                    print!("{}", choice.delta.content.as_ref().unwrap());
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Create edit

Creates a new edit for the provided input, instruction, and parameters.

**URL** `https://api.openai.com/v1/edits`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::edit::EditParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditParameters {
        model: "text-davinci-edit-001".to_string(),
        input: "What day of the wek is it?".to_string(),
        instruction: "Fix the spelling mistakes".to_string(),
        temperature: None,
    };

    let result = client.edits().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create edit](https://platform.openai.com/docs/api-reference/edits/create)

### Create image

Creates an image given a prompt.

**URL** `https://api.openai.com/v1/images/generations`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageParameters {
        prompt: "A cute baby dog".to_string(),
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let result = client.images().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)

### Create image edit

Creates an edited or extended image given an original image and a prompt.

**URL** `https://api.openai.com/v1/images/edits`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EditImageParameters {
        image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
        mask: Some("./images/image_edit_mask.png".to_string()), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_mask.png
        prompt: "A cute baby sea otter wearing a beret".to_string(),
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/create-edit)

### Create image variation

Creates a variation of a given image.

**URL** `https://api.openai.com/v1/images/variations`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageVariationParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateImageVariationParameters {
        image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let result = client.images().variation(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create image variation](https://platform.openai.com/docs/api-reference/images/create-variation)

### Create embedding

Creates an embedding vector representing the input text.

**URL** `https://api.openai.com/v1/embeddings`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::embedding::EmbeddingParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParameters {
        model: "text-embedding-ada-002".to_string(),
        input: "The food was delicious and the waiter...".to_string(),
    };

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create embedding](https://platform.openai.com/docs/api-reference/embeddings/create)

### Create transcription

Transcribes audio into the input language.

**URL** `https://api.openai.com/v1/audio/transcriptions`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioTranscriptionParameters, AudioTranscriptOutputFormat};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranscriptionParameters {
        file: "./audio/micro-machines.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/micro-machines.mp3
        model: "whisper-1".to_string(),
        prompt: None,
        response_format: Some(AudioTranscriptOutputFormat::Srt),
        temperature: None,
        language: None,
    };

    let result = client.audio().create_transcription(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create transcription](https://platform.openai.com/docs/api-reference/audio/create)

### Create translation

Translates audio into English.

**URL** `https://api.openai.com/v1/audio/translations`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::audio::{AudioTranscriptOutputFormat, AudioTranslationParameters};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranslationParameters {
        file: "./audio/multilingual.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/multilingual.mp3
        model: "whisper-1".to_string(),
        prompt: None,
        response_format: Some(AudioTranscriptOutputFormat::Srt),
        temperature: None,
    };

    let result = client.audio().create_translation(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create translation](https://platform.openai.com/docs/api-reference/audio/create)

### List files

Returns a list of files that belong to the user's organization.

**URL** `https://api.openai.com/v1/files`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.files().list().await.unwrap();

    println!("{:?}", result);
}
```

More information: [List files](https://platform.openai.com/docs/api-reference/files/list)

### Upload file

Upload a file that contains document(s) to be used across various endpoints/features.

**URL** `https://api.openai.com/v1/files`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::UploadFileParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = UploadFileParameters {
        file: "./files/SentimentAnalysisSample.jsonl".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/SentimentAnalysisSample.jsonl
        purpose: "fine-tune".to_string(), // currently the only supported purpose by OpenAI is `fine-tune`
    };

    let result = client.files().upload(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Upload file](https://platform.openai.com/docs/api-reference/files/upload)

### Delete file

Delete a file.

**URL** `https://api.openai.com/v1/files/{file_id}`

**Method** `DELETE`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.files().delete("file-XXX").await.unwrap();

    println!("{:?}", result);
}
```

More information: [Delete file](https://platform.openai.com/docs/api-reference/files/delete)

### Retrieve file

Returns information about a specific file.

**URL** `https://api.openai.com/v1/files/{file_id}`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.files().retrieve("file-XXX").await.unwrap();

    println!("{:?}", result);
}
```

More information: [Retrieve file](https://platform.openai.com/docs/api-reference/files/retrieve)

### Retrieve file content

> **Note**
> To help mitigate abuse, downloading of fine-tune training files is disabled for free accounts.

Returns the contents of the specified file.

**URL** `https://api.openai.com/v1/files/{file_id}/content`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let result = client.files().retrieve_content("file-XXX").await.unwrap();

    println!("{:?}", result);
}
```

More information: [Retrieve file content](https://platform.openai.com/docs/api-reference/files/retrieve-content)

### Create moderation

Classifies if text violates OpenAI's Content Policy.

**URL** `https://api.openai.com/v1/files/{file_id}/content`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::moderation::ModerationParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ModerationParameters {
        input: "I want to kill them.".to_string(),
        model: "text-moderation-latest".to_string(),
    };

    let result = client.moderations().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create moderation](https://platform.openai.com/docs/api-reference/moderations)

## Set API key

Add the OpenAI API key to your environment variables.

```sh
# Windows PowerShell
$Env:OPENAI_API_KEY='sk-...'

# Windows cmd
set OPENAI_API_KEY=sk-...

# Linux/macOS
export OPENAI_API_KEY='sk-...'
```

## Add proxy

This crate uses `reqwest` as HTTP Client. Reqwest has proxies enabled by default. You can set the proxy via the system environment variable or by overriding the default client.

### Example: set system environment variable

You can set the proxy in the system environment variables ([https://docs.rs/reqwest/latest/reqwest/#proxies](https://docs.rs/reqwest/latest/reqwest/#proxies)).

```sh
export HTTPS_PROXY=socks5://127.0.0.1:1086
```

### Example: overriding the default client

```rust
let http_client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::https("socks5://127.0.0.1:1086")?)
    .build()?;

let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

let client = Client {
    http_client,
    base_url: "https://api.openai.com/v1".to_string(),
    api_key,
};
```
