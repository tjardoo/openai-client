# OpenAI Dive

OpenAI Dive is an async Rust API client that allows you to interact with the OpenAI API.

```ini
[dependencies]
openai_dive = "0.1"
```

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
  - [Edit image](#edit-image)
  - [Image variation](#image-variation)
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

### List models

Lists the currently available models, and provides basic information about each one such as the owner and availability.

**URL** `https://api.openai.com/v1/models`

**Method** `GET`

```rust
use openai_dive::v1::api::Client;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let model_id = OpenAIModel::TextDavinci003.to_string(); // text-davinci-003

    let client = Client::new(api_key);

    let result = client.models().get(model_id).await.unwrap();

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = CompletionParameters {
        model: OpenAIModel::TextDavinci003.to_string(), // text-davinci-003
        prompt: "Say this is a test".to_string(),
        suffix: None,
        max_tokens: 10,
        temperature: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = CompletionParameters {
        model: OpenAIModel::TextDavinci003.to_string(), // text-davinci-003
        prompt: "Create an outline for an essay about Nikola Tesla and his contributions to technology:".to_string(),
        suffix: None,
        max_tokens: 100,
        temperature: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = ChatCompletionParameters {
        model: OpenAIModel::Chat3X5Turbo0301.to_string(), // gpt-3.5-turbo-0301
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        max_tokens: 12,
        temperature: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = ChatCompletionParameters {
        model: OpenAIModel::Chat3X5Turbo0301.to_string(), // gpt-3.5-turbo-0301
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        max_tokens: 12,
        temperature: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = EditParameters {
        model: OpenAIModel::TextDavinciEdit001.to_string(), // text-davinci-edit-001
        input: "What day of the wek is it?".to_string(),
        instruction: "Fix the spelling mistakes".to_string(),
        temperature: None,
    };

    let client = Client::new(api_key);

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
    let api_key = "YOUR API KEY".to_string();

    let parameters = CreateImageParameters {
        prompt: "A cute baby dog".to_string(),
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let client = Client::new(api_key);

    let result = client.images().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)

### Edit image

Creates an edited or extended image given an original image and a prompt.

**URL** `https://api.openai.com/v1/images/edits`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = EditImageParameters {
        image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
        mask: Some("./images/image_edit_mask.png".to_string()), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_mask.png
        prompt: "A cute baby sea otter weaing a beret".to_string(),
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let client = Client::new(api_key);

    let result = client.images().edit(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/create-edit)

### Image variation

Creates a variation of a given image.

**URL** `https://api.openai.com/v1/images/variations`

**Method** `POST`

```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{CreateImageVariationParameters, ImageSize};

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = CreateImageVariationParameters {
        image: "./images/image_edit_original.png".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/image_edit_original.png
        number_of_images: Some(1),
        image_size: Some(ImageSize::Size256X256),
        response_format: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;
use openai_dive::v1::resources::embedding::EmbeddingParameters;

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = EmbeddingParameters {
        model: OpenAIModel::TextEmbeddingAda002.to_string(), // text-embedding-ada-002
        input: "The food was delicious and the waiter...".to_string(),
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;
use openai_dive::v1::resources::audio::{AudioTranscriptOutputFormat, AudioTranslationParameters};

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = AudioTranslationParameters {
        file: "./audio/micro-machines.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/micro-machines.mp3
        model: OpenAIModel::Whisper1.to_string(),
        prompt: None,
        response_format: Some(AudioTranscriptOutputFormat::Srt),
        temperature: None,
    };

    let client = Client::new(api_key);

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
use openai_dive::v1::models::OpenAIModel;
use openai_dive::v1::resources::audio::{AudioTranscriptOutputFormat, AudioTranslationParameters};

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY".to_string();

    let parameters = AudioTranslationParameters {
        file: "./audio/multilingual.mp3".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/multilingual.mp3
        model: OpenAIModel::Whisper1.to_string(),
        prompt: None,
        response_format: Some(AudioTranscriptOutputFormat::Srt),
        temperature: None,
    };

    let client = Client::new(api_key);

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
    let api_key = "YOUR API KEY".to_string();

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
    let api_key = "YOUR API KEY".to_string();

    let parameters = UploadFileParameters {
        file: "./files/SentimentAnalysisSample.jsonl".to_string(), // https://github.com/betalgo/openai/blob/master/OpenAI.Playground/SampleData/SentimentAnalysisSample.jsonl
        purpose: "fine-tune".to_string(), // currently the only supported purpose by OpenAI is `fine-tune`
    };

    let client = Client::new(api_key);

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
    let api_key = "YOUR API KEY".to_string();

    let client = Client::new(api_key);

    let result = client.files().delete("file-XXX".to_string()).await.unwrap();

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
    let api_key = "YOUR API KEY".to_string();

    let client = Client::new(api_key);

    let result = client.files().retrieve("file-XXX".to_string()).await.unwrap();

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
    let api_key = "YOUR API KEY".to_string();

    let client = Client::new(api_key);

    let result = client.files().retrieve_content("file-XXX".to_string()).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Retrieve file content](https://platform.openai.com/docs/api-reference/files/retrieve-content)
