#[cfg(feature = "tokio")]
use crate::v1::error::APIError;
use crate::v1::resources::shared::FileUpload;
use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
#[cfg(feature = "tokio")]
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "AudioSpeechParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct AudioSpeechParameters {
    /// One of the available TTS models: tts-1 or tts-1-hd.
    pub model: String,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,
    /// The voice to use when generating the audio.
    pub voice: AudioVoice,
    /// Control the voice of your generated audio with additional instructions. Does not work with tts-1 or tts-1-hd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// The format to audio in. Supported formats are mp3, opus, aac, flac, wav and pcm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioSpeechResponseFormat>,
    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "AudioTranscriptionParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct AudioTranscriptionParameters {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: FileUpload,
    /// ID of the model to use. Only whisper-1 is currently available.
    pub model: String,
    /// The language of the input audio. Supplying the input language in ISO-639-1 format will improve accuracy and latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Controls how the audio is cut into chunks. When set to "auto", the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. server_vad object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<TranscriptionChunkingStrategy>,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioOutputFormat>,
    /// If set to true, the model response data will be streamed to the client as it is generated using server-sent events. Note: Streaming is not supported for the whisper-1 model and will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    /// If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// The timestamp granularities to populate for this transcription. response_format must be set verbose_json to use timestamp granularities.
    /// Either or both of these options are supported: word, or segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
    /// Allows to pass arbitrary json as an extra_body parameter, for specific features/openai-compatible endpoints.
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "AudioTranslationParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct AudioTranslationParameters {
    /// The audio file object to translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: FileUpload,
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
    /// The voice to use when generating the audio.
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
#[serde(rename_all = "snake_case")]
pub enum AudioOutputFormat {
    Json,
    Text,
    Srt,
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AudioVoice {
    #[default]
    Alloy,
    Ash,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimestampGranularity {
    Word,
    Segment,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionChunkingStrategy {
    Auto,
    #[serde(untagged)]
    VadConfig(VadConfig),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct VadConfig {
    /// Must be set to "server_vad" to enable manual chunking using server side VAD.
    pub r#type: VadConfigType,
    /// Amount of audio to include before the VAD detected speech (in milliseconds).
    pub prefix_padding_ms: Option<usize>,
    /// Duration of silence to detect speech stop (in milliseconds). With shorter values the model will respond more quickly, but may jump in on short pauses from the user.
    pub silence_duration_ms: Option<usize>,
    /// Sensitivity threshold (0.0 to 1.0) for voice activity detection. A higher threshold will require louder audio to activate the model, and thus might perform better in noisy environments.
    pub threshold: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VadConfigType {
    ServerVad,
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

impl Display for TimestampGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimestampGranularity::Word => "word",
                TimestampGranularity::Segment => "segment",
            }
        )
    }
}

impl Display for TranscriptionChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscriptionChunkingStrategy::Auto => "auto".fmt(f),
            TranscriptionChunkingStrategy::VadConfig(vad_config) => vad_config.fmt(f),
        }
    }
}

impl Display for VadConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
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

#[cfg(test)]
mod tests {
    use crate::v1::resources::audio::{
        AudioTranscriptionParameters, AudioTranscriptionParametersBuilder,
        TranscriptionChunkingStrategy, VadConfig, VadConfigType,
    };
    use crate::v1::resources::shared::FileUpload;

    #[test]
    fn test_audio_transcription_chunking_strategy_auto_serialization_deserialization() {
        let chunking_strategy = TranscriptionChunkingStrategy::Auto;

        let serialized = serde_json::to_string(&chunking_strategy).unwrap();
        assert_eq!(serialized, "\"auto\"");

        let deserialized: TranscriptionChunkingStrategy =
            serde_json::from_str(serialized.as_str()).unwrap();
        assert_eq!(deserialized, chunking_strategy)
    }

    #[test]
    fn test_audio_transcription_chunking_strategy_vad_config_serialization_deserialization() {
        let chunking_strategy = TranscriptionChunkingStrategy::VadConfig(VadConfig {
            r#type: VadConfigType::ServerVad,
            prefix_padding_ms: Some(10),
            silence_duration_ms: Some(20),
            threshold: Some(0.5),
        });

        let serialized = serde_json::to_string(&chunking_strategy).unwrap();
        assert_eq!(serialized, "{\"type\":\"server_vad\",\"prefix_padding_ms\":10,\"silence_duration_ms\":20,\"threshold\":0.5}");

        let deserialized: TranscriptionChunkingStrategy =
            serde_json::from_str(serialized.as_str()).unwrap();
        assert_eq!(deserialized, chunking_strategy)
    }

    #[test]
    fn test_audio_transcription_extra_body_serialization_deserialization() {
        let mut builder = &mut AudioTranscriptionParametersBuilder::default();
        builder = builder.file(FileUpload::File("test.wav".to_string()));
        builder = builder.model("test");
        let extra = serde_json::json!({
            "enable_my_feature": true,
            "my_param": 10
        });
        builder = builder.extra_body(extra);

        let params: AudioTranscriptionParameters = builder.build().unwrap();

        let serialized = serde_json::to_string(&params).unwrap();
        assert_eq!(serialized, "{\"file\":{\"File\":\"test.wav\"},\"model\":\"test\",\"enable_my_feature\":true,\"my_param\":10}");

        let deserialized: AudioTranscriptionParameters =
            serde_json::from_str(serialized.as_str()).unwrap();
        assert_eq!(deserialized, params)
    }
}
