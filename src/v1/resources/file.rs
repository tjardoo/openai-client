use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The size of the file, in bytes.
    pub bytes: u32,
    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: u32,
    /// The name of the file.
    pub filename: String,
    /// The object type, which is always file.
    pub object: String,
    /// The intended purpose of the file. Supported values are fine-tune, fine-tune-results, assistants, and assistants_output.
    pub purpose: FilePurpose,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFilesParameters {
    /// Only return files with the given purpose.
    pub purpose: Option<FilePurpose>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFilesResponse {
    pub data: Vec<File>,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UploadFileParameters {
    pub file: String,
    pub purpose: FilePurpose,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeletedFile {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
}

impl Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FilePurpose::FineTune => "fine-tune",
                FilePurpose::FineTuneResults => "fine-tune-results",
                FilePurpose::Assistants => "assistants",
                FilePurpose::AssistantsOutput => "assistants_output",
            }
        )
    }
}
