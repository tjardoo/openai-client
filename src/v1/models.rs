use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter, Result};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, 
            "{}",
            serde_json::to_string(self).map_err(|_| Error)?
        )
    }
}
