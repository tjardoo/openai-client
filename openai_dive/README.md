# OpenAI Dive

[![crates.io](https://img.shields.io/crates/v/openai_dive.svg?style=flat-square)](https://crates.io/crates/openai_dive)
![cargo build](https://img.shields.io/github/actions/workflow/status/tjardoo/openai-client/cargo-build.yml?style=flat-square)
[![docs.rs](https://img.shields.io/docsrs/openai_dive?style=flat-square)](https://docs.rs/openai_dive)
[![crates.io](https://img.shields.io/crates/d/openai_dive.svg?style=flat-square)](https://crates.io/crates/openai_dive)

OpenAI Dive is an unofficial async Rust library that allows you to interact with the OpenAI API.

```ini
[dependencies]
openai_dive = "1.4"
```

## Get started

```rust
use openai_dive::v1::api::Client;

let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

let client = Client::new_from_env(); // or Client::new(api_key);

let result = client
    .models()
    .list()
    .await?;
```

- [Set API key](#set-api-key)
- [Using OpenAI-compatible APIs](#using-openai-compatible-apis)
- [Set organization/project id](#set-organizationproject-id)
- [Add proxy](#add-proxy)
- [Available models](#available-models)

## Endpoints

- [Chat](#chat)
  - [Completion](#completion)
  - [Vision](#vision)
  - [Voice](#voice)
  - [Function calling](#function-calling)
  - [Structured outputs](#structured-outputs)
  - [Web search](#web-search)
- [Responses](#responses)
- [Images](#images)
- [Audio](#audio)
- [Videos](#videos)
- [Models](#models)
- [Files](#files)
- [Embeddings](#embeddings)
- [Moderation](#moderation)
- [Uploads](#uploads)
- [Fine-tuning](#fine-tuning)
- [Batches](#batches)
- [Administration](#administration)
- [Usage](#usage)
- [Realtime](#realtime)

## Chat

Given a list of messages comprising a conversation, the model will return a response.

### Completion

Creates a model response for the given chat conversation.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Model::Gpt4O.to_string())
    .messages(vec![
        ChatMessage::User {
            content: ChatMessageContent::Text("Hello!".to_string()),
            name: None,
        },
        ChatMessage::User {
            content: ChatMessageContent::Text("What is the capital of Vietnam?".to_string()),
            name: None,
        },
    ])
    .response_format(ChatCompletionResponseFormat::Text)
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Vision

Learn how to use vision capabilities to understand images.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Model::Gpt4O.to_string())
    .messages(vec![
        ChatMessage::User {
            content: ChatMessageContent::Text("What is in this image?".to_string()),
            name: None,
        },
        ChatMessage::User {
            content: ChatMessageContent::ContentPart(vec![ChatMessageContentPart::Image(
                ChatMessageImageContentPart {
                    r#type: "image_url".to_string(),
                    image_url: ImageUrlType {
                        url:
                            "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640"
                                .to_string(),
                        detail: None,
                    },
                },
            )]),
            name: None,
        },
    ])
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;
```

More information: [Vision](https://platform.openai.com/docs/guides/vision)

### Voice

Learn how to use audio capabilities to understand audio files.

```rust
let recording = std::fs::read("example-audio.txt").unwrap();

let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Model::Gpt4OAudioPreview.to_string())
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
                    data: String::from_utf8(recording).unwrap(),
                    format: "mp3".to_string(),
                },
            }]),
            name: None,
        },
    ])
    .build()?;

let result = client
    .chat()
    .create(parameters)
    .await?;
```

More information: [Vision](https://platform.openai.com/docs/guides/audio)

### Function calling

In an API call, you can describe functions and have the model intelligently choose to output a JSON object containing arguments to call one or many functions. The Chat Completions API does not call the function; instead, the model generates JSON that you can use to call the function in your code.

```rust
let messages = vec![ChatMessage::User {
    content: ChatMessageContent::Text(
        "Give me a random number higher than 100 but less than 2*150?".to_string(),
    ),
    name: None,
}];

let parameters = ChatCompletionParametersBuilder::default()
    .model(Gpt4Model::Gpt4O.to_string())
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

if let ChatMessage::Assistant {
    tool_calls: Some(tool_calls),
    ..
} = message
{
    for tool_call in tool_calls {
        let name = tool_call.function.name;
        let arguments = tool_call.function.arguments;

        if name == "get_random_number" {
            let random_numbers: RandomNumber = serde_json::from_str(&arguments).unwrap();

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

### Structured outputs

Structured Outputs is a feature that guarantees the model will always generate responses that adhere to your supplied JSON Schema, so you don't need to worry about the model omitting a required key, or hallucinating an invalid enum value.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model("gpt-4o-2024-08-06")
    .messages(vec![
        ChatMessage::System {
            content: ChatMessageContent::Text(
                "You are a helpful math tutor. Guide the user through the solution step by step."
                    .to_string(),
            ),
            name: None,
        },
        ChatMessage::User {
            content: ChatMessageContent::Text(
                "How can I solve 8x + 7 = -23"
                    .to_string(),
            ),
            name: None,
        },
    ])
    .response_format(ChatCompletionResponseFormat::JsonSchema {
        json_schema: JsonSchemaBuilder::default()
            .name("math_reasoning")
            .schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "steps": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "explanation": { "type": "string" },
                                "output": { "type": "string" }
                            },
                            "required": ["explanation", "output"],
                            "additionalProperties": false
                        }
                    },
                    "final_answer": { "type": "string" }
                },
                "required": ["steps", "final_answer"],
                "additionalProperties": false
            }))
            .strict(true)
            .build()?
        }
    )
    .build()?;

let result = client.chat().create(parameters).await?;
```

More information: [Structured outputs](https://platform.openai.com/docs/guides/structured-outputs)

### Web search

Allow models to search the web for the latest information before generating a response.

```rust
let parameters = ChatCompletionParametersBuilder::default()
    .model(ToolModel::Gpt4OMiniSearchPreview.to_string())
    .messages(vec![ChatMessage::User {
        content: ChatMessageContent::Text(
            "What was a positive news story from today?!".to_string(),
        ),
        name: None,
    }])
    .web_search_options(WebSearchOptions {
        search_context_size: Some(WebSearchContextSize::Low),
        user_location: Some(ApproximateUserLocation {
            r#type: UserLocationType::Approximate,
            approximate: WebSearchUserLocation {
                city: Some("Amsterdam".to_string()),
                country: Some("NL".to_string()),
                region: None,
                timezone: None,
            },
        }),
    })
    .response_format(ChatCompletionResponseFormat::Text)
    .build()?;

let result = client.chat().create(parameters).await?;
```

More information: [Web search](https://platform.openai.com/docs/guides/web-search)

## Responses

OpenAI's most advanced interface for generating model responses. Supports text and image inputs, and text outputs. Create stateful interactions with the model, using the output of previous responses as input. Extend the model's capabilities with built-in tools for file search, web search, computer use, and more. Allow the model access to external systems and data using function calling.

For more information see the examples in the [examples/responses](https://github.com/tjardoo/openai-client/tree/master/examples/responses) directory.

- Text & image inputs
- Text outputs
- Stateful interactions
- File search
- Web search
- Computer use
- Function calling

## Images

Given a prompt and/or an input image, the model will generate a new image.

- Create image
- Create image edit
- Create image variation

For more information see the examples in the [examples/images](https://github.com/tjardoo/openai-client/tree/master/examples/images) directory.

More information [Images](https://platform.openai.com/docs/api-reference/images)

## Audio

Learn how to turn audio into text or text into audio.

- Create speech
- Create transcription
- Create translation

For more information see the examples in the [examples/audio](https://github.com/tjardoo/openai-client/tree/master/examples/audio) directory.

More information [Audio](https://platform.openai.com/docs/api-reference/audio)

## Videos

Create and remix videos.

- Create video
- Create remix
- List videos
- Retrieve video
- Delete video
- Retrieve video content

For more information see the examples in the [examples/video](https://github.com/tjardoo/openai-client/tree/master/examples/video) directory.

More information [Video](https://platform.openai.com/docs/api-reference/videos)

## Models

List and describe the various models available in the API.

For more information see the examples in the [examples/models](https://github.com/tjardoo/openai-client/tree/master/examples/models) directory.

- List models
- Retrieve model
- Delete fine-tune model

More information [Models](https://platform.openai.com/docs/api-reference/models)

## Files

Files are used to upload documents that can be used with features like Assistants, Fine-tuning, and Batch API.

For more information see the examples in the [examples/files](https://github.com/tjardoo/openai-client/tree/master/examples/files) directory.

- List files
- Upload file
- Delete file
- Retrieve file
- Retrieve file content

More information [Files](https://platform.openai.com/docs/api-reference/files)

## Embeddings

Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.

For more information see the examples in the [examples/embeddings](https://github.com/tjardoo/openai-client/tree/master/examples/embeddings) directory.

- Create embeddings

More information: [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)

## Moderation

Given some input text, outputs if the model classifies it as potentially harmful across several categories.

For more information see the examples in the [examples/moderations](https://github.com/tjardoo/openai-client/tree/master/examples/moderations) directory.

- Create moderation

More information [Moderation](https://platform.openai.com/docs/api-reference/moderations)

## Uploads

Creates an intermediate Upload object that you can add Parts to. Currently, an Upload can accept at most 8 GB in total and expires after an hour after you create it.

Once you complete the Upload, we will create a File object that contains all the parts you uploaded. This File is usable in the rest of our platform as a regular File object.

For more information see the examples in the [examples/uploads](https://github.com/tjardoo/openai-client/tree/master/examples/uploads) directory.

- Create upload
- Add upload part
- Complete upload
- Cancel upload

More information [Uploads](https://platform.openai.com/docs/api-reference/uploads)

## Fine-tuning

Manage fine-tuning jobs to tailor a model to your specific training data.

For more information see the examples in the [examples/fine_tuning](https://github.com/tjardoo/openai-client/tree/master/examples/fine_tuning) directory.

- Create fine-tuning job
- List fine-tuning jobs
- Retrieve fine-tuning job
- Cancel fine-tuning job
- List fine-tuning events
- List fine-tuning checkpoints

More information [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)

## Batches

Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.

For more information see the examples in the [examples/batches](https://github.com/tjardoo/openai-client/tree/master/examples/batches) directory.

- Create batch
- List batches
- Retrieve batch
- Cancel batch

More information [Batch](https://platform.openai.com/docs/api-reference/batch)

## Administration

Programmatically manage your organization.

For more information see the examples in the [examples/administration](https://github.com/tjardoo/openai-client/tree/master/examples/administration) directory.

- Users
- Invites
- Projects
- Project Users
- Project Service Accounts
- Project API Keys
- Rate Limits
- Audit Logs

More information [Administration](https://platform.openai.com/docs/api-reference/administration)

## Usage

The Usage API provides detailed insights into your activity across the OpenAI API.

It also includes a separate Costs endpoint, which offers visibility into your spend, breaking down consumption by invoice line items and project IDs.

For more information see the examples in the [examples/usage](https://github.com/tjardoo/openai-client/tree/master/examples/usage) directory.

- Completions
- Embeddings
- Moderations
- Images
- Audio speeches
- Audio transcriptions
- Vector stores
- Code interpreter sessions
- Costs

More information [Usage](https://platform.openai.com/docs/api-reference/usage)

## Realtime

Communicate with a GPT-4o class model live, in real time, over WebSocket. Produces both audio and text transcriptions.

Enable the feature flag `realtime` to use this feature.

For more information see the examples in the [examples/realtime](https://github.com/tjardoo/openai-client/tree/master/examples/realtime) directory.

- All client events
- All server events

More information [Realtime](https://platform.openai.com/docs/api-reference/realtime)

## Configuration

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

### Using OpenAI-compatible APIs

By simply changing the base URL, you can use this crate with other OpenAI-compatible APIs.

```rust
let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY is not set");

let mut client = Client::new(deepseek_api_key);
client.set_base_url("https://api.deepseek.com");
```

Use `extra_body` in `ChatCompletionParameters` to pass non-standard parameters supported by OpenAI-compatible APIs.

Use `query_params` in `ChatCompletionParameters` to pass non-standard `query` parameters supported by OpenAI-compatible APIs.

### Set organization/project ID

You can create multiple organizations and projects in the OpenAI platform. This allows you to group files, fine-tuned models and other resources.

You can set the organization ID and/or project ID on the client via the `set_organization` and `set_project` methods. If you don't set the organization and/or project ID, the client will use the default organization and default project.

```rust
let mut client = Client::new_from_env();

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
    headers: None,
    organization: None,
    project: None,
};
```

### Available Models

You can use these predefined constants to set the model in the parameters or use any string representation (ie. for your custom models).

#### GPT-5

- Gpt51 (`gpt-5.1`)
- Gpt5O (`gpt-5o`)
- Gpt5OAudioPreview (`gpt-5o-audio-preview`)

#### GPT-4

- Gpt41 (`gpt-4.1`)
- Gpt4O (`gpt-4o`)
- Gpt4OAudioPreview (`gpt-4o-audio-preview`)

#### Tool Models

- Gpt4OSearchPreview (`gpt-4o-search-preview`)
- Gpt4OMiniSearchPreview (`gpt-4o-mini-search-preview`)
- ComputerUsePreview (`computer-use-preview`)

#### Moderation Models

- OmniModerationLatest (`omni-moderation-latest`)

#### Embedding Models

- TextEmbedding3Small (`text-embedding-3-small`)
- TextEmbedding3Large (`text-embedding-3-large`)

#### Transcription Models

- Gpt4OTranscribe (`gpt-4o-transcribe`)
- Whisper1 (`whisper-1`)

#### TTS Models

- Gpt4OMiniTts (`gpt-4o-mini-tts`)
- Tts1 (`tts-1`)
- Tts1HD (`tts-1-hd`)

#### Image Models

- GptImage1 (`gpt-image-1`)
- DallE3 (`dall-e-3`)
- DallE2 (`dall-e-2`)

#### Video Models

- Sora2 (`sora-2`)
- Sora2Pro (`sora-2-pro`)

More information: [Models](https://platform.openai.com/docs/models)
