use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text {
        /// The text content.
        text: String,
    },
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data.
        audio: Option<String>,
        /// The transcript of the audio.
        transcript: String,
    },
}
