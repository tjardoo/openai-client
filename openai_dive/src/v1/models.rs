use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FlagshipModel {
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4o")]
    Gpt4O,
    #[serde(rename = "gpt-4o-audio-preview")]
    Gpt4OAudioPreview,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CostOptimizedModel {
    #[serde(rename = "o4-mini")]
    O4Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4OMini,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ReasoningModel {
    #[serde(rename = "o4-mini")]
    O4Mini,
    #[serde(rename = "o3-mini")]
    O3Mini,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ToolModel {
    #[serde(rename = "gpt-4o-search-preview")]
    Gpt4OSearchPreview,
    #[serde(rename = "gpt-4o-mini-search-preview")]
    Gpt4OMiniSearchPreview,
    #[serde(rename = "computer-use-preview")]
    ComputerUsePreview,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModerationModel {
    #[serde(rename = "omni-moderation-latest")]
    OmniModerationLatest,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TranscriptionModel {
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4OTranscribe,
    #[serde(rename = "whisper-1")]
    Whisper1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TTSModel {
    #[serde(rename = "gpt-4o-mini-tts")]
    Gpt4OMiniTts,
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1HD,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImageModel {
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "dall-e-3")]
    DallE3,
    #[serde(rename = "dall-e-2")]
    DallE2,
}

macro_rules! impl_display {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let json_str = serde_json::to_string(self).unwrap();
                let trimmed_str = json_str.trim_matches('"');
                write!(f, "{}", trimmed_str)
            }
        }
    };
}

impl_display!(FlagshipModel);
impl_display!(CostOptimizedModel);
impl_display!(ReasoningModel);
impl_display!(ToolModel);
impl_display!(ModerationModel);
impl_display!(EmbeddingModel);
impl_display!(TranscriptionModel);
impl_display!(TTSModel);
impl_display!(ImageModel);
