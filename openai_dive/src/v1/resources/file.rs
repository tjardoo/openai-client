use crate::v1::resources::shared::FileUpload;
use derive_builder::Builder;
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
    /// The object type, which is always "file".
    pub object: String,
    /// The intended purpose of the file.
    pub purpose: FilePurpose,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListFilesParameters {
    /// Only return files with the given purpose.
    pub purpose: Option<FilePurpose>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "UploadFileParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct UploadFileParameters {
    /// The File object to be uploaded.
    pub file: FileUpload,
    /// The intended purpose of the uploaded file.
    pub purpose: FilePurpose,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    Assistants,
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    Vision,
    #[default]
    UserData,
    Evals,
}

impl Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FilePurpose::Assistants => "assistants",
                FilePurpose::Batch => "batch",
                FilePurpose::FineTune => "fine-tune",
                FilePurpose::Vision => "vision",
                FilePurpose::UserData => "user_data",
                FilePurpose::Evals => "evals",
            }
        )
    }
}
