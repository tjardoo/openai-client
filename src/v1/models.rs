use std::fmt::Display;
use serde::{Serialize, Deserialize};

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
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3_5Turbo0613,
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
        write!(f, "{}",
            match self {
                OpenAIModel::Gpt4 => "gpt-4",
                OpenAIModel::Gpt4_0613 => "gpt-4-0613",
                OpenAIModel::Gpt4_32K => "gpt-4-32k",
                OpenAIModel::Gpt4_32K0613 => "gpt-4-32k-0613",
                OpenAIModel::Gpt3_5Turbo => "gpt-3.5-turbo-0301",
                OpenAIModel::Gpt3_5Turbo0613 => "gpt-3.5-turbo-0613",
                OpenAIModel::TextEmbeddingAda002 => "text-embedding-ada-002",
                OpenAIModel::Whisper1 => "whisper-1",
                OpenAIModel::TextModerationStable => "text-moderation-stable",
                OpenAIModel::TextModerationLatest => "text-moderation-latest",
            }
        )
    }
}
