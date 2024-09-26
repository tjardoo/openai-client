use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ModerationParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ModerationParameters {
    /// Input (or inputs) to classify.
    pub input: ModerationInput,
    /// The content moderation model you would like to use.
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationInput {
    /// A string of text to classify for moderation.
    Text(String),
    /// An array of strings to classify for moderation.
    Array(Vec<String>),
    /// An array of multi-modal inputs to the moderation model.
    MultiModal(Vec<ModerationObject>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationObject {
    Text {
        r#type: String,
        text: String,
    },
    ImageUrl {
        r#type: String,
        /// Contains either an image URL or a data URL for a base64 encoded image.
        image_url: ModerationImageUrl,
    },
}

impl ModerationObject {
    pub fn text(text: &str) -> Self {
        ModerationObject::Text {
            r#type: "text".to_string(),
            text: text.to_string(),
        }
    }

    pub fn image_url(url: &str) -> Self {
        ModerationObject::ImageUrl {
            r#type: "image_url".to_string(),
            image_url: ModerationImageUrl {
                url: url.to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModerationImageUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModerationResponse {
    /// The unique identifier for the moderation request.
    pub id: String,
    /// The model used to generate the moderation results.
    pub model: String,
    /// A list of moderation objects.
    pub results: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Results {
    /// Whether any of the below categories are flagged.
    pub flagged: bool,
    /// A list of the categories, and whether they are flagged or not.
    pub categories: Categories,
    /// A list of the categories along with their scores as predicted by model.
    pub category_scores: CategoryScores,
    /// A list of the categories along with the input type(s) that the score applies to.
    pub category_applied_input_types: CategoryAppliedInputTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Categories {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    pub harassment: bool,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    pub illicit: bool,
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CategoryScores {
    pub hate: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    pub harassment: f64,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    pub illicit: f64,
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CategoryAppliedInputTypes {
    pub hate: Vec<String>,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: Vec<String>,
    pub harassment: Vec<String>,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: Vec<String>,
    pub illicit: Vec<String>,
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: Vec<String>,
    #[serde(rename = "self-harm")]
    pub self_harm: Vec<String>,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: Vec<String>,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: Vec<String>,
    pub sexual: Vec<String>,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: Vec<String>,
    pub violence: Vec<String>,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: Vec<String>,
}

impl Default for ModerationInput {
    fn default() -> Self {
        ModerationInput::Text("".to_string())
    }
}
