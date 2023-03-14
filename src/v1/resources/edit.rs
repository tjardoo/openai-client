use serde::{Serialize, Deserialize};
use super::shared::Usage;

#[derive(Serialize, Debug)]
pub struct EditParameters {
    pub model: String,
    pub input: String,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditResponse {
    pub object: String,
    pub created: u32,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditChoice {
    pub index: u32,
    pub text: String,
}
