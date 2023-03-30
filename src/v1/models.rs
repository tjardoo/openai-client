use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-3.5-turbo")]
    Chat3X5Turbo,
    #[serde(rename = "gpt-3.5-turbo-0301")]
    Chat3X5Turbo0301,
    #[serde(rename = "text-davinci-003")]
    TextDavinci003,
    #[serde(rename = "text-davinci-edit-001")]
    TextDavinciEdit001,
    #[serde(rename = "text-curie-001")]
    TextCurie001,
    #[serde(rename = "text-babbage-001")]
    TextBabbage001,
    #[serde(rename = "text-ada-001")]
    TextAda001,
}

impl Display for OpenAIModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAIModel::Chat3X5Turbo0301 => f.write_str("gpt-3.5-turbo-0301"),
            OpenAIModel::Chat3X5Turbo => f.write_str("gpt-3.5-turbo"),
            OpenAIModel::TextDavinci003 => f.write_str("text-davinci-003"),
            OpenAIModel::TextDavinciEdit001 => f.write_str("text-davinci-edit-001"),
            OpenAIModel::TextCurie001 => f.write_str("text-curie-001"),
            OpenAIModel::TextBabbage001 => f.write_str("text-babbage-001"),
            OpenAIModel::TextAda001 => f.write_str("text-ada-001"),
        }
    }
}
