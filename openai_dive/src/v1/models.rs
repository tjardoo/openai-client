use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum O1Engine {
    // Alias
    #[serde(rename = "o1")]
    O1,
    /// Alias
    #[serde(rename = "o1-mini")]
    O1Mini,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt4Engine {
    /// Alias
    #[serde(rename = "gpt-4")]
    Gpt4,
    /// Alias
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    /// Alias
    #[serde(rename = "gpt-4o")]
    Gpt4O,
    /// Alias
    #[serde(rename = "gpt-4o-mini")]
    Gpt4OMini,
    /// Alias
    #[serde(rename = "gpt-4o-realtime-preview")]
    Gpt4ORealtimePreview,
    /// Alias
    #[serde(rename = "gpt-4o-mini-realtime-preview")]
    Gpt4OMiniRealtimePreview,
    /// Alias
    #[serde(rename = "gpt-4o-audio-preview")]
    Gpt4OAudioPreview,
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
    #[serde(rename = "omni-moderation-latest")]
    OmniModerationLatest,
    /// Alias
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest,
    /// Alias
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeepSeekEngine {
    /// Alias
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    /// Alias
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

impl Display for O1Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            O1Engine::O1 => write!(f, "o1"),
            O1Engine::O1Mini => write!(f, "o1-mini"),
        }
    }
}

impl Display for Gpt4Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gpt4Engine::Gpt4 => write!(f, "gpt-4"),
            Gpt4Engine::Gpt4Turbo => write!(f, "gpt-4-turbo"),
            Gpt4Engine::Gpt4O => write!(f, "gpt-4o"),
            Gpt4Engine::Gpt4OMini => write!(f, "gpt-4o-mini"),
            Gpt4Engine::Gpt4OAudioPreview => write!(f, "gpt-4o-audio-preview"),
            Gpt4Engine::Gpt4ORealtimePreview => write!(f, "gpt-4o-realtime-preview"),
            Gpt4Engine::Gpt4OMiniRealtimePreview => write!(f, "gpt-4o-mini-realtime-preview"),
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
            ModerationsEngine::OmniModerationLatest => write!(f, "omni-moderation-latest"),
            ModerationsEngine::TextModerationLatest => write!(f, "text-moderation-latest"),
            ModerationsEngine::TextModerationStable => write!(f, "text-moderation-stable"),
        }
    }
}

impl Display for DeepSeekEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DeepSeekEngine::DeepSeekChat => write!(f, "deepseek-chat"),
            DeepSeekEngine::DeepSeekReasoner => write!(f, "deepseek-reasoner"),
        }
    }
}
