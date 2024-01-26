use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt4Engine {
    /// Alias
    #[serde(rename = "gpt-4")]
    Gpt4,
    /// Alias
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4-32k")]
    Gpt432K,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432K0613,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt35Engine {
    /// Alias
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
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
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModerationsEngine {
    /// Alias
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest,
    /// Alias
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
    #[serde(rename = "text-moderation-007")]
    TextModeration007,
}

impl Display for Gpt4Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gpt4Engine::Gpt4 => write!(f, "gpt-4"),
            Gpt4Engine::Gpt4TurboPreview => write!(f, "gpt-4-turbo-preview"),
            Gpt4Engine::Gpt40125Preview => write!(f, "gpt-4-0125-preview"),
            Gpt4Engine::Gpt40613 => write!(f, "gpt-4-0613"),
            Gpt4Engine::Gpt41106Preview => write!(f, "gpt-4-1106-preview"),
            Gpt4Engine::Gpt432K => write!(f, "gpt-4-32k"),
            Gpt4Engine::Gpt432K0613 => write!(f, "gpt-4-32k-0613"),
            Gpt4Engine::Gpt4VisionPreview => write!(f, "gpt-4-vision-preview"),
        }
    }
}

impl Display for Gpt35Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gpt35Engine::Gpt35Turbo => write!(f, "gpt-3.5-turbo"),
            Gpt35Engine::Gpt35Turbo0125 => write!(f, "gpt-3.5-turbo-0125"),
            Gpt35Engine::Gpt35Turbo1106 => write!(f, "gpt-3.5-turbo-1106"),
            Gpt35Engine::Gpt35Turbo16K => write!(f, "gpt-3.5-turbo-16k"),
            Gpt35Engine::Gpt35TurboInstruct => write!(f, "gpt-3.5-turbo-instruct"),
        }
    }
}

impl Display for DallEEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DallEEngine::DallE2 => write!(f, "dall-e-2"),
            DallEEngine::DallE3 => write!(f, "dall-e-3"),
        }
    }
}

impl Display for TTSEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TTSEngine::Tts1 => write!(f, "tts-1"),
            TTSEngine::Tts1HD => write!(f, "tts-1-hd"),
        }
    }
}

impl Display for WhisperEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            WhisperEngine::Whisper1 => write!(f, "whisper-1"),
        }
    }
}

impl Display for EmbeddingsEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EmbeddingsEngine::TextEmbedding3Small => write!(f, "text-embedding-3-small"),
            EmbeddingsEngine::TextEmbedding3Large => write!(f, "text-embedding-3-large"),
            EmbeddingsEngine::TextEmbeddingAda002 => write!(f, "text-embedding-ada-002"),
        }
    }
}

impl Display for ModerationsEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ModerationsEngine::TextModerationLatest => write!(f, "text-moderation-latest"),
            ModerationsEngine::TextModerationStable => write!(f, "text-moderation-stable"),
            ModerationsEngine::TextModeration007 => write!(f, "text-moderation-007"),
        }
    }
}
