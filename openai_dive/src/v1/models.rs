use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt5Model {
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-mini")]
    Gpt5Mini,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gpt4Model {
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4o")]
    Gpt4O,
    #[serde(rename = "gpt-4o-audio-preview")]
    Gpt4OAudioPreview,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VideoModel {
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "sora-2-pro")]
    Sora2Pro,
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

impl_display!(Gpt5Model);
impl_display!(Gpt4Model);
impl_display!(ToolModel);
impl_display!(ModerationModel);
impl_display!(EmbeddingModel);
impl_display!(TranscriptionModel);
impl_display!(TTSModel);
impl_display!(ImageModel);
impl_display!(VideoModel);
