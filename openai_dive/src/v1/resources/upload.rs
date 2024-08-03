use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{
    file::{File, FilePurpose},
    shared::FileUpload,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Upload {
    /// The Upload unique identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the Upload was created.
    pub created_at: u32,
    /// The name of the file to be uploaded.
    pub filename: String,
    /// The intended number of bytes to be uploaded.
    pub bytes: u32,
    /// The intended purpose of the file.
    pub purpose: FilePurpose,
    /// The status of the Upload.
    pub status: UploadStatus,
    /// The Unix timestamp (in seconds) for when the Upload was created.
    pub expires_at: u32,
    /// The object type, which is always "upload".
    pub object: String,
    /// The File object represents a document that has been uploaded to OpenAI.
    pub file: Option<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UploadPart {
    /// The upload Part unique identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the Part was created.
    pub created_at: u32,
    /// The ID of the Upload object that this Part was added to.
    pub upload_id: String,
    /// The object type, which is always upload.part.
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CreateUploadParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CreateUploadParameters {
    /// The name of the file to upload.
    pub filename: String,
    /// The intended purpose of the uploaded file.
    pub purpose: FilePurpose,
    /// The number of bytes in the file you are uploading.
    pub bytes: u64,
    /// The MIME type of the file.
    pub mime_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
pub struct AddPartParameters {
    /// The chunk of bytes for this Part.
    pub data: FileUpload,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "CompleteUploadParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct CompleteUploadParameters {
    /// The ordered list of Part IDs.
    pub part_ids: Vec<String>,
    /// The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UploadStatus {
    Pending,
    Completed,
    Cancelled,
}
