use crate::v1::error::APIError;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
#[cfg(feature = "download")]
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) async fn check_status_code(
    result: reqwest::Result<Response>,
) -> Result<Response, APIError> {
    match result {
        Ok(response) => {
            if response.status().is_client_error() {
                let status = response.status();
                let text = response.text().await.unwrap();

                match status {
                    StatusCode::BAD_REQUEST => {
                        return Err(APIError::BadRequestError(text));
                    }
                    StatusCode::UNAUTHORIZED => {
                        return Err(APIError::AuthenticationError(text));
                    }
                    StatusCode::FORBIDDEN => {
                        return Err(APIError::PermissionError(text));
                    }
                    StatusCode::NOT_FOUND => {
                        return Err(APIError::NotFoundError(text));
                    }
                    StatusCode::GONE => {
                        return Err(APIError::GoneError(text));
                    }
                    StatusCode::TOO_MANY_REQUESTS => {
                        return Err(APIError::RateLimitError(text));
                    }
                    _ => {
                        return Err(APIError::UnknownError(status.as_u16(), text));
                    }
                }
            }

            Ok(response)
        }
        Err(error) => Err(APIError::ServerError(error.to_string())),
    }
}

pub(crate) fn validate_response(response: String) -> Result<Value, APIError> {
    let value: Value =
        serde_json::from_str(&response).map_err(|error| APIError::ParseError(error.to_string()))?;

    if let Some(object) = value.as_object() {
        if object.len() == 1 && object.contains_key("error") {
            return Err(APIError::InvalidRequestError(value["error"].to_string()));
        }
    }

    Ok(value)
}

pub(crate) fn format_response<R: DeserializeOwned>(response: String) -> Result<R, APIError> {
    let value = validate_response(response)?;

    let value: R =
        serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

    Ok(value)
}

pub(crate) fn is_beta_feature(path: &str) -> bool {
    let beta_features = ["/assistants", "/threads", "/vector_stores"];

    beta_features
        .iter()
        .any(|feature| path.starts_with(feature))
}

#[cfg(feature = "download")]
pub fn generate_file_name(path: &str, length: u32, file_type: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let alphabet_len = ALPHABET.len();

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let mut seed = since_the_epoch.as_nanos();

    let mut random_str = String::with_capacity(length as usize);

    for _ in 0..length {
        let index = (seed % alphabet_len as u128) as usize;
        random_str.push(ALPHABET[index] as char);
        seed /= alphabet_len as u128;
    }

    format!("{}/{}.{}", path, random_str, file_type)
}
