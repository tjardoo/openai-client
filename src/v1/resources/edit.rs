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

impl Default for EditParameters {
    fn default() -> Self {
        EditParameters {
            model: "text-davinci-edit-001".to_string(),
            input: Some("What day of the wek is it?".to_string()),
            instruction: "Fix the spelling mistakes".to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }
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
