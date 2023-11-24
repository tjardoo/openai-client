use crate::v1::error::APIError;
#[cfg(feature = "download")]
use rand::{distributions::Alphanumeric, Rng};
use reqwest::multipart::Part;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub fn validate_request(response: String) -> Result<Value, APIError> {
    let value: Value = serde_json::from_str(&response).unwrap();

    if Value::is_object(&value["error"]) {
        return Err(APIError::InvalidRequestError(value["error"].to_string()));
    }

    return Ok(value);
}

pub fn format_request<R: DeserializeOwned>(response: String) -> Result<R, APIError> {
    let value = validate_request(response)?;

    let value: R =
        serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

    return Ok(value);
}

pub fn is_beta_feature(path: &str) -> bool {
    path.starts_with("/assistants") || path.starts_with("/threads")
}

pub async fn file_from_disk_to_form_part(path: String) -> Result<Part, APIError> {
    let file = File::open(&path)
        .await
        .map_err(|error| APIError::FileError(error.to_string()))?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = reqwest::Body::wrap_stream(stream);

    let file_part = reqwest::multipart::Part::stream(file_body)
        .file_name(path)
        .mime_str("application/octet-stream")
        .unwrap();

    Ok(file_part)
}

#[cfg(feature = "download")]
pub fn generate_file_name(path: &str, length: u32, file_type: &str) -> String {
    let random_file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();

    format!("{}/{}.{}", path, random_file_name, file_type)
}
