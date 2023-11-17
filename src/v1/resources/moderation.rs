use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModerationParameters {
    pub input: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Results {
    pub categories: Categories,
    pub category_scores: CategoryScores,
    pub flagged: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Categories {
    pub sexual: bool,
    pub hate: bool,
    pub harassment: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    pub violence: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CategoryScores {
    pub sexual: f64,
    pub hate: f64,
    pub harassment: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    pub violence: f64,
}
