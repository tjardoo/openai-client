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

## Get started

```rust
use openai_dive::v1::api::Client;

let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

let client = Client::new(api_key);

let result = client
    .models()
    .list()
    .await?;
```

- [Set API key](#set-api-key)
- [Set organization/project id](#set-organizationproject-id)
- [Add proxy](#add-proxy)
- [Available models](#available-models)

## Endpoints

- [Models](#models)
  - [List models](#list-models)
  - [Retrieve model](#retrieve-model)
  - [Delete fine-tune model](#delete-fine-tune-model)
- [Chat](#chat)
  - [Create chat completion](#create-chat-completion)
  - [Create chat completion with image](#create-chat-completion-with-image)
  - [Function calling](#function-calling)
- [Images](#images)
  - [Create image](#create-image)
  - [Create image edit](#create-image-edit)
  - [Create image variation](#create-image-variation)
- [Audio](#audio)
  - [Create speech](#create-speech)
  - [Create transcription](#create-transcription)
  - [Create translation](#create-translation)
- [Embeddings](#embeddings)
  - [Create embeddings](#create-embeddings)
- [Files](#files)
  - [List files](#list-files)
  - [Upload file](#upload-file)
  - [Delete file](#delete-file)
  - [Retrieve file](#retrieve-file)
  - [Retrieve file content](#retrieve-file-content)
- [Moderation](#moderation)
  - [Create moderation](#create-moderation)
- [Fine-tuning](#fine-tuning)
- [Batches](#batches)
- [Assistants](#assistants)
  - [Currency Converter Assistant](#currency-converter-assistant)

## Models

List and describe the various models available in the API.

### List models

Lists the currently available models, and provides basic information about each one such as the owner and availability.

```rust
let result = client
    .models()
    .list()
    .await?;
```

More information: [List models](https://platform.openai.com/docs/api-reference/models/list)

### Retrieve model

Retrieves a model instance, providing basic information about the model such as the owner and permissioning.

```rust
let result = client
    .models()
    .get("gpt-4o")
    .await?;
```

More information: [Retrieve model](https://platform.openai.com/docs/api-reference/models/retrieve)

### Delete fine-tune model

Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.

```rust
let result = client
    .models()
    .delete("my-custom-model")
    .await?;
```

More information: [Delete fine-tune model](https://platform.openai.com/docs/api-reference/models/delete)

## Chat

Given a list of messages comprising a conversation, the model will return a response.

### Create chat completion

Creates a model response for the given chat conversation.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/chat/create_chat_completion_stream](https://github.com/tjardoo/openai-client/tree/master/examples/chat/create_chat_completion_stream) example.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Engine::Gpt4O.to_string())
    .messages(vec![
        ChatMessageBuilder::default()
            .role(Role::User)
            .content(ChatMessageContent::Text("Hello!".to_string()))
            .build()?,
        ChatMessageBuilder::default()
            .role(Role::User)
            .content(ChatMessageContent::Text(
                "What is the capital of Vietnam?".to_string(),
            ))
            .build()?,
    ])
    .response_format(ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::Text,
    })
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Create chat completion with image

Creates a model response for the given chat conversation.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Engine::Gpt4O.to_string())
    .messages(vec![
        ChatMessageBuilder::default()
            .role(Role::User)
            .content(ChatMessageContent::Text(
                "What is in this image?".to_string(),
            ))
            .build()?,
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
            .build()?,
    ])
    .max_tokens(50u32)
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Function calling

In an API call, you can describe functions and have the model intelligently choose to output a JSON object containing arguments to call one or many functions. The Chat Completions API does not call the function; instead, the model generates JSON that you can use to call the function in your code.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/chat/function_calling_stream](https://github.com/tjardoo/openai-client/tree/master/examples/chat/function_calling_stream) example.

```rust
let messages = vec![ChatMessageBuilder::default()
    .content(ChatMessageContent::Text(
        "Give me a random number between 100 and no more than 150?".to_string(),
    ))
    .build()?];

let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Engine::Gpt4O.to_string())
    .messages(messages)
    .tools(vec![ChatCompletionTool {
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
    }])
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;

let message = result.choices[0].message.clone();

if let Some(tool_calls) = message.tool_calls {
    for tool_call in tool_calls {
        let name = tool_call.function.name;
        let arguments = tool_call.function.arguments;

        if name == "get_random_number" {
            let random_numbers: RandomNumber = serde_json::from_str(&arguments)?;

            println!("Min: {:?}", &random_numbers.min);
            println!("Max: {:?}", &random_numbers.max);

            let random_number_result = get_random_number(random_numbers);

            println!(
                "Random number between those numbers: {:?}",
                random_number_result.clone()
            );
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
let parameters = CreateImageParametersBuilder::default()
    .prompt("A cute dog in the park")
    .model(DallEEngine::DallE3.to_string())
    .n(1u32)
    .quality(ImageQuality::Standard)
    .response_format(ResponseFormat::Url)
    .size(ImageSize::Size1024X1024)
    .style(ImageStyle::Natural)
    .build()?;

let result = client
    .images()
    .create(parameters)
    .await?;

let paths = result
    .save("./images")
    .await?;
```

More information: [Create image](https://platform.openai.com/docs/api-reference/images/create)

### Create image edit

Creates an edited or extended image given an original image and a prompt.

```rust
let parameters = EditImageParametersBuilder::default()
    .image("./images/image_edit_original.png")
    .prompt("A cute baby sea otter")
    .mask("./images/image_edit_mask.png")
    .n(1u32)
    .size(ImageSize::Size512X512)
    .build()?;

let result = client
    .images()
    .edit(parameters)
    .await?;
```

More information: [Create image edit](https://platform.openai.com/docs/api-reference/images/createEdit)

### Create image variation

Creates a variation of a given image.

```rust
let parameters = CreateImageVariationParametersBuilder::default()
    .image("./images/image_edit_original.png")
    .n(1u32)
    .size(ImageSize::Size256X256)
    .build()?;

let result = client
    .images()
    .variation(parameters)
    .await?;
```

More information: [Create image variation](https://platform.openai.com/docs/api-reference/images/createVariation)

## Audio

Learn how to turn audio into text or text into audio.

### Create speech

Generates audio from the input text.

> [!NOTE]
> This endpoint also has `stream` support. See the [examples/audio/create_speech_stream](https://github.com/tjardoo/openai-client/tree/master/examples/audio/create_speech_stream) example.

```rust
let parameters = AudioSpeechParametersBuilder::default()
    .model(TTSEngine::Tts1.to_string())
    .input("Hallo, this is a test from OpenAI Dive.")
    .voice(AudioVoice::Alloy)
    .response_format(AudioSpeechResponseFormat::Mp3)
    .build()?;

let response = client
    .audio()
    .create_speech(parameters)
    .await?;

response
    .save("files/example.mp3")
    .await?;
```

More information: [Create speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)

### Create transcription

Transcribes audio into the input language.

```rust
let parameters = AudioTranscriptionParametersBuilder::default()
    .file(AudioTranscriptionFile::File(
        "./audio/micro-machines.mp3".to_string(),
    ))
    .model(WhisperEngine::Whisper1.to_string())
    .response_format(AudioOutputFormat::VerboseJson)
    .build()?;

let result = client
    .audio()
    .create_transcription(parameters)
    .await?;
```

More information: [Create transcription](https://platform.openai.com/docs/api-reference/audio/createTranscription)

### Create translation

Translates audio into English.

```rust
let parameters = AudioTranslationParametersBuilder::default()
    .file("./audio/multilingual.mp3")
    .model(WhisperEngine::Whisper1.to_string())
    .response_format(AudioOutputFormat::Srt)
    .build()?;

let result = client
    .audio()
    .create_translation(parameters)
    .await?;
```

More information: [Create translation](https://platform.openai.com/docs/api-reference/audio/createTranslation)

## Embeddings

Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.

### Create embeddings

Creates an embedding vector representing the input text.

```rust
let parameters = EmbeddingParametersBuilder::default()
    .model(EmbeddingsEngine::TextEmbedding3Small.to_string())
    .input(EmbeddingInput::String(
        "The food was delicious and the waiter...".to_string(),
    ))
    .encoding_format(EmbeddingEncodingFormat::Float)
    .build()?;

let result = client
    .embeddings()
    .create(parameters)
    .await?;
```

More information: [Create embeddings](https://platform.openai.com/docs/api-reference/embeddings/create)

## Files

Files are used to upload documents that can be used with features like Assistants, Fine-tuning, and Batch API.

### List files

Returns a list of files that belong to the user's organization.

```rust
let query = ListFilesParameters {
    purpose: Some(FilePurpose::FineTune),
};

let result = client
    .files()
    .list(Some(query))
    .await?;
```

More information: [List files](https://platform.openai.com/docs/api-reference/files/list)

### Upload file

Upload a file that can be used across various endpoints.

```rust
let parameters = UploadFileParametersBuilder::default()
    .file("./files/DummyUsers.json")
    .purpose(FilePurpose::Assistants)
    .build()?;

let result = client
    .files()
    .upload(parameters)
    .await?;
```

More information [Upload file](https://platform.openai.com/docs/api-reference/files/create)

### Delete file

Delete a file.

```rust
    let file_id = "file-XXX";

    let result = client
        .files()
        .delete(&file_id)
        .await?;
```

More information [Delete file](https://platform.openai.com/docs/api-reference/files/delete)

### Retrieve file

Returns information about a specific file.

```rust
let file_id = "file-XXX";

let result = client
    .files()
    .retrieve(&file_id)
    .await?;
```

More information [Retrieve file](https://platform.openai.com/docs/api-reference/files/retrieve)

### Retrieve file content

Returns the contents of the specified file.

```rust
let file_id = "file-XXX";

let result = client
    .files()
    .retrieve_content(&file_id)
    .await?;
```

More information [Retrieve file content](https://platform.openai.com/docs/api-reference/files/retrieve-contents)

## Moderation

Given some input text, outputs if the model classifies it as potentially harmful across several categories.

### Create moderation

Classifies if text is potentially harmful.

```rust
let parameters = ModerationParametersBuilder::default()
    .model(ModerationsEngine::TextModerationLatest.to_string())
    .input("I want to kill them.".to_string())
    .build()?;

let result = client
    .moderations()
    .create(parameters)
    .await?;
```

More information [Create moderation](https://platform.openai.com/docs/api-reference/moderations/create)

### Fine-tuning

Manage fine-tuning jobs to tailor a model to your specific training data.

For more information see the examples in the [examples/fine_tuning](https://github.com/tjardoo/openai-client/tree/master/examples/fine_tuning) directory.

- Create fine-tuning job
- List fine-tuning jobs
- Retrieve fine-tuning job
- Cancel fine-tuning job
- List fine-tuning events
- List fine-tuning checkpoints

More information [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)

### Batches

Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.

For more information see the examples in the [examples/batches](https://github.com/tjardoo/openai-client/tree/master/examples/batches) directory.

- Create batch
- List batches
- Retrieve batch
- Cancel batch

More information [Batch](https://platform.openai.com/docs/api-reference/batch)

## Assistants

Build assistants that can call models and use tools to perform tasks.

For more information see the examples in the [examples/assistants](https://github.com/tjardoo/openai-client/tree/master/examples/assistants) directory.

- Assistants
- Threads
- Messages
- Runs
- Run Steps

More information [Assistants](https://platform.openai.com/docs/api-reference/assistants)

### Currency Converter Assistant

The [`Currency Converter Assistant`](https://github.com/tjardoo/openai-client/tree/master/examples/assistants) processes conversion queries. It integrates a function `"get_currency_conversion"` that calls an external API endpoint to fetch real-time conversion rates with EUR as the base. The assistant is set up by creating a thread and run via the `create_thread_and_run` endpoint, checking the run's status, and handling tool outputs when required. Finally, it retrieves the assistant's responses using the `list_messages` endpoint to display the conversion results.

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

### Set organization/project ID

You can create multiple organizations and projects in the OpenAI platform. This allows you to group files, fine-tuned models and other resources.

You can set the organization ID and/or project ID on the client via the `set_organization` and `set_project` methods. If you don't set the organization and/or project ID, the client will use the default organization and default project.

```rust
let mut client = Client::new(api_key);

client
    .set_organization("org-XXX")
    .set_project("proj_XXX");
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
    organization: None,
};
```

### Available Models

You can use these predefined constants to set the model in the parameters or use any string representation (ie. for your custom models).

- Gpt4Engine
  - Gpt4O `gpt-4o` (alias)
  - Gpt4 `gpt-4` (alias)
  - Gpt4Turbo `gpt-4-turbo` (alias)
  - Gpt4TurboPreview `gpt-4-turbo-preview` (alias)
- Gpt35Engine
  - Gpt35Turbo `gpt-3.5-turbo` (alias)
  - Gpt35Turbo1106 `gpt-3.5-turbo-1106`
- DallEEngine
  - DallE3 `dall-e-2`
  - DallE2 `dall-e-3`
- TTSEngine
  - Tts1 `tts-1`
  - Tts1HD `tts-1-hd`
- WhisperEngine
  - Whisper1 `whisper-1`
- EmbeddingsEngine
  - TextEmbedding3Small `text-embedding-3-small`
  - TextEmbedding3Large `text-embedding-3-large`
  - TextEmbeddingAda002 `text-embedding-ada-002`
- ModerationsEngine
  - TextModerationLatest `text-moderation-latest` (alias)
  - TextModerationStable `text-moderation-stable` (alias)
  - TextModeration007 `text-moderation-007`

More information: [Models](https://platform.openai.com/docs/models)
