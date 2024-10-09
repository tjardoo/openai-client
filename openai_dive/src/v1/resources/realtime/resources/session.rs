use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Session {
    /// The default model used for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The set of modalities the model can respond with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,
    /// The default system instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// The voice the model uses to respond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    /// The format of input audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<AudioFormat>,
    /// The format of output  audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<AudioFormat>,
    /// Configuration for input audio transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,
    /// Configuration for turn detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<TurnDetection>,
    /// Tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// How the model chooses tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of output tokens (integer or "inf").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioTranscription {
    /// Whether input audio transcription is enabled.
    pub enabled: bool,
    /// The model used for transcription.
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum TurnDetection {
    #[serde(rename = "server_vad")]
    ServerVAD {
        /// Activation threshold for VAD.
        threshold: f32,
        /// Audio included before speech starts (in milliseconds).
        prefix_padding_ms: u32,
        /// Duration of silence to detect speech stop (in milliseconds).
        silence_duration_ms: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Tool {
    #[serde(rename = "function")]
    Function {
        /// The name of the function.
        name: String,
        /// The description of the function.
        description: String,
        /// Parameters of the function in JSON Schema.
        parameters: serde_json::Value,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    Alloy,
    Echo,
    Shimmer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711-ulaw")]
    G711Ulaw,
    #[serde(rename = "g711-alaw")]
    G711Alaw,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    #[serde(untagged)]
    Function {
        /// Function type to use.
        r#type: FunctionType,
        /// Function name to use.
        name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MaxOutputTokens {
    Integer(u32),
    #[serde(rename = "inf")]
    Inf,
}
