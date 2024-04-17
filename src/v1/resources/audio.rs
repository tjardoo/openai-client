use crate::v1::error::APIError;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioSpeechParameters {
    /// One of the available TTS models: tts-1 or tts-1-hd.
    pub model: String,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,
    /// The voice to use when generating the audio. Supported voices are alloy, echo, fable, onyx, nova, and shimmer.
    pub voice: AudioVoice,
    /// The format to audio in. Supported formats are mp3, opus, aac, flac, wav and pcm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioSpeechResponseFormat>,
    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioTranscriptionParameters {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    #[serde(skip)]
    pub file: AudioTranscriptionFile,
    /// ID of the model to use. Only whisper-1 is currently available.
    pub model: String,
    /// The language of the input audio. Supplying the input language in ISO-639-1 format will improve accuracy and latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioOutputFormat>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    /// If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// The timestamp granularities to populate for this transcription. response_format must be set verbose_json to use timestamp granularities.
    /// Either or both of these options are supported: word, or segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioTranslationParameters {
    /// The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: String,
    /// ID of the model to use. Only whisper-1 is currently available.
    pub model: String,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should be in English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioOutputFormat>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    /// If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct AudioSpeechResponse {
    pub bytes: Bytes,
}

#[cfg(feature = "stream")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamAudioSpeechParameters {
    /// One of the available TTS models: tts-1 or tts-1-hd.
    pub model: String,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,
    /// The voice to use when generating the audio. Supported voices are alloy, echo, fable, onyx, nova, and shimmer.
    pub voice: AudioVoice,
    /// The format to audio in. Supported formats are mp3, opus, aac, flac, wav and pcm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioSpeechResponseFormat>,
    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    pub stream: bool,
}

#[cfg(feature = "stream")]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioSpeechResponseChunkResponse {
    pub bytes: Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AudioOutputFormat {
    Json,
    Text,
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    Vtt,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AudioSpeechResponseFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
    Wav,
    Pcm,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioTranscriptionBytes {
    pub bytes: Bytes,
    pub filename: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioTranscriptionFile {
    Bytes(AudioTranscriptionBytes),
    File(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AudioVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimestampGranularity {
    Word,
    Segment,
}

impl Display for AudioOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AudioOutputFormat::Json => "json",
                AudioOutputFormat::Text => "text",
                AudioOutputFormat::Srt => "srt",
                AudioOutputFormat::VerboseJson => "verbose_json",
                AudioOutputFormat::Vtt => "vtt",
            }
        )
    }
}

impl AudioSpeechResponse {
    #[cfg(feature = "tokio")]
    pub async fn save<P: AsRef<Path>>(&self, file_path: P) -> Result<(), APIError> {
        let directory = file_path.as_ref().parent();

        if let Some(directory) = directory {
            let is_existing_directory = match Path::try_exists(directory) {
                Ok(exists) => exists,
                Err(error) => return Err(APIError::FileError(error.to_string())),
            };

            if !is_existing_directory {
                std::fs::create_dir_all(directory)
                    .map_err(|error| APIError::FileError(error.to_string()))?;
            }
        }

        tokio::fs::write(file_path, &self.bytes)
            .await
            .map_err(|error| APIError::FileError(error.to_string()))?;

        Ok(())
    }
}

impl Default for AudioTranscriptionFile {
    fn default() -> Self {
        Self::File(String::new())
    }
}
