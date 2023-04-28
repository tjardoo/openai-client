use serde::{Serialize, Deserialize};
use crate::v1::resources::shared::Usage;

#[derive(Serialize, Debug, Clone)]
pub struct EditParameters {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditResponse {
    pub object: String,
    pub created: u32,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditChoice {
    pub index: u32,
    pub text: String,
}
