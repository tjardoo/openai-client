use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: u32,
    pub created_at: u32,
    pub filename: String,
    pub purpose: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileParameters {
    pub file: String,
    pub purpose: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedFile {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
