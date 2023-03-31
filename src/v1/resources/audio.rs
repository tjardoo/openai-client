use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct AudioTranscriptionParameters {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioTranscriptOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AudioTranslationParameters {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioTranscriptOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AudioTranscriptOutputFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

impl Display for AudioTranscriptOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                AudioTranscriptOutputFormat::Json => "json",
                AudioTranscriptOutputFormat::Text => "text",
                AudioTranscriptOutputFormat::Srt => "srt",
                AudioTranscriptOutputFormat::VerboseJson => "verbose_json",
                AudioTranscriptOutputFormat::Vtt => "vtt",
            }
        )
    }
}
