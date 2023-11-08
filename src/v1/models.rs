use serde::{Deserialize, Serialize};
use std::fmt::Display;

// https://platform.openai.com/docs/models/overview
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0613")]
    Gpt4_0613,
    #[serde(rename = "gpt-4-32k")]
    Gpt4_32K,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt4_32K0613,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt4_1106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3_5Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3_5Turbo1106,
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
    #[serde(rename = "whisper-1")]
    Whisper1,
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest,
}

impl Display for OpenAIModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpenAIModel::Gpt4 => "gpt-4",
                OpenAIModel::Gpt4_0613 => "gpt-4-0613",
                OpenAIModel::Gpt4_32K => "gpt-4-32k",
                OpenAIModel::Gpt4_32K0613 => "gpt-4-32k-0613",
                OpenAIModel::Gpt4_1106Preview => "gpt-4-1106-preview",
                OpenAIModel::Gpt4VisionPreview => "gpt-4-vision-preview",
                OpenAIModel::Gpt3_5Turbo => "gpt-3.5-turbo-0301",
                OpenAIModel::Gpt3_5Turbo0613 => "gpt-3.5-turbo-0613",
                OpenAIModel::Gpt3_5Turbo1106 => "gpt-3.5-turbo-1106",
                OpenAIModel::TextEmbeddingAda002 => "text-embedding-ada-002",
                OpenAIModel::Whisper1 => "whisper-1",
                OpenAIModel::TextModerationStable => "text-moderation-stable",
                OpenAIModel::TextModerationLatest => "text-moderation-latest",
            }
        )
    }
}
