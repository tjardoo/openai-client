#[cfg(feature = "reqwest")]
use crate::v1::error::APIError;
use bytes::Bytes;
#[cfg(feature = "reqwest")]
use reqwest::{header::HeaderMap, multipart::Part};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
    /// Number of tokens in the entire response.
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseWrapper<T> {
    pub data: T,
    pub headers: Headers,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Headers {
    /// The maximum number of requests that are permitted before exhausting the rate limit.
    #[serde(rename = "x-ratelimit-limit-requests")]
    pub x_ratelimit_limit_requests: Option<u32>,
    /// The maximum number of tokens that are permitted before exhausting the rate limit.
    #[serde(rename = "x-ratelimit-limit-tokens")]
    pub x_ratelimit_limit_tokens: Option<u32>,
    /// The remaining number of requests that are permitted before exhausting the rate limit.
    #[serde(rename = "x-ratelimit-remaining-requests")]
    pub x_ratelimit_remaining_requests: Option<u32>,
    /// The remaining number of tokens that are permitted before exhausting the rate limit.
    #[serde(rename = "x-ratelimit-remaining-tokens")]
    pub x_ratelimit_remaining_tokens: Option<u32>,
    /// The time until the rate limit (based on requests) resets to its initial state.
    #[serde(rename = "x-ratelimit-reset-requests")]
    pub x_ratelimit_reset_requests: Option<String>,
    /// The time until the rate limit (based on tokens) resets to its initial state.
    #[serde(rename = "x-ratelimit-reset-tokens")]
    pub x_ratelimit_reset_tokens: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimpleListParameters {
    /// Identifier for the last object from the previous pagination request.
    pub after: Option<String>,
    /// Number of objects to retrieve.
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListParameters {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the created_at timestamp of the objects. asc for ascending order and desc for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// A cursor for use in pagination. after is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include after=obj_foo in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. before is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include before=obj_foo in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListResponse<T> {
    // The object type, which is always "list".
    pub object: String,
    /// The list ob objects.
    pub data: Vec<T>,
    /// The ID of the first objects in the list.
    pub first_id: Option<String>,
    /// The ID of the last objects in the list.
    pub last_id: Option<String>,
    /// Indicates whether there are more objects to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeletedObject {
    /// ID of the deleted object.
    pub id: String,
    /// The object type.
    pub object: String,
    /// Indicates whether the file was successfully deleted.
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// API returned complete message, or a message terminated by one of the stop sequences provided via the stop parameter.
    #[serde(rename = "stop")]
    StopSequenceReached,
    /// Incomplete model output due to max_tokens parameter or token limit.
    #[serde(rename = "length")]
    TokenLimitReached,
    /// Omitted content due to a flag from our content filters.
    #[serde(rename = "content_filter")]
    ContentFilterFlagged,
    /// The model decided to call one or more tools.
    ToolCalls,
    /// The model reached a natural stopping point. [Claude]
    EndTurn,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum StopToken {
    String(String),
    Array(Vec<String>),
}

#[cfg(feature = "reqwest")]
impl From<HeaderMap> for Headers {
    fn from(value: HeaderMap) -> Self {
        if value.get("x-ratelimit-limit-requests").is_none()
            || value.get("x-ratelimit-limit-tokens").is_none()
            || value.get("x-ratelimit-remaining-requests").is_none()
            || value.get("x-ratelimit-remaining-tokens").is_none()
            || value.get("x-ratelimit-reset-requests").is_none()
            || value.get("x-ratelimit-reset-tokens").is_none()
        {
            return Self {
                x_ratelimit_limit_requests: None,
                x_ratelimit_limit_tokens: None,
                x_ratelimit_remaining_requests: None,
                x_ratelimit_remaining_tokens: None,
                x_ratelimit_reset_requests: None,
                x_ratelimit_reset_tokens: None,
            };
        }

        Self {
            x_ratelimit_limit_requests: Some(
                value
                    .get("x-ratelimit-limit-requests")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            ),
            x_ratelimit_limit_tokens: Some(
                value
                    .get("x-ratelimit-limit-tokens")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            ),
            x_ratelimit_remaining_requests: Some(
                value
                    .get("x-ratelimit-remaining-requests")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            ),
            x_ratelimit_remaining_tokens: Some(
                value
                    .get("x-ratelimit-remaining-tokens")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            ),
            x_ratelimit_reset_requests: Some(
                value
                    .get("x-ratelimit-reset-requests")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
            x_ratelimit_reset_tokens: Some(
                value
                    .get("x-ratelimit-reset-tokens")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileUploadBytes {
    pub bytes: Bytes,
    pub filename: String,
}
impl FileUploadBytes {
    pub fn new(bytes: impl Into<Bytes>, filename: impl Into<String>) -> Self {
        Self {
            bytes: bytes.into(),
            filename: filename.into(),
        }
    }

    #[cfg(feature = "reqwest")]
    pub(crate) fn into_part(self) -> Result<Part, APIError> {
        reqwest::multipart::Part::bytes(self.bytes.to_vec())
            .file_name(self.filename.clone())
            .mime_str("application/octet-stream")
            .map_err(|error| APIError::FileError(error.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FileUpload {
    Bytes(FileUploadBytes),
    #[cfg(all(feature = "tokio", feature = "tokio-util"))]
    File(String),
}
impl FileUpload {
    #[cfg(feature = "reqwest")]
    pub(crate) async fn into_part(self) -> Result<Part, APIError> {
        match self {
            FileUpload::Bytes(bytes) => bytes.into_part(),
            #[cfg(all(feature = "tokio", feature = "tokio-util"))]
            FileUpload::File(path) => {
                use tokio::fs::File;
                use tokio_util::codec::{BytesCodec, FramedRead};

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
        }
    }
}
impl Default for FileUpload {
    fn default() -> Self {
        Self::Bytes(FileUploadBytes::new(Bytes::new(), ""))
    }
}
