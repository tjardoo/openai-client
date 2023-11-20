use crate::v1::error::APIError;
use reqwest::multipart::Part;
use serde_json::Value;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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

pub async fn validate_request(response: String) -> Result<Value, APIError> {
    let value: Value = serde_json::from_str(&response).unwrap();

    if Value::is_object(&value["error"]) {
        return Err(APIError::InvalidRequestError(value["error"].to_string()));
    }

    return Ok(value);
}
