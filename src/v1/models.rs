use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt4Engine {
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-32k")]
    Gpt432K,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432K0613,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt35Engine {
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16K,
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    Gpt35TurboInstruct,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DallEEngine {
    #[serde(rename = "dall-e-3")]
    DallE3,
    #[serde(rename = "dall-e-2")]
    DallE2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TTSEngine {
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1HD,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WhisperEngine {
    #[serde(rename = "whisper-1")]
    Whisper1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EmbeddingsEngine {
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModerationsEngine {
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest,
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
}

impl Display for Gpt4Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for Gpt35Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for DallEEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for TTSEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for WhisperEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for EmbeddingsEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

impl Display for ModerationsEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}
