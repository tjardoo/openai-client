use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum APIError {
    AuthenticationError(String),
    BadRequestError(String),
    PermissionError(String),
    NotFoundError(String),
    GoneError(String),
    ServerError(String),
    InvalidRequestError(String),
    RateLimitError(String),
    ParseError(String),
    FileError(String),
    StreamError(String),
    WebSocketError(String),
    UnknownError(u16, String),
}

impl APIError {
    fn message(&self) -> String {
        match self {
            APIError::AuthenticationError(message)
            | APIError::BadRequestError(message)
            | APIError::PermissionError(message)
            | APIError::NotFoundError(message)
            | APIError::GoneError(message)
            | APIError::ServerError(message)
            | APIError::InvalidRequestError(message)
            | APIError::RateLimitError(message)
            | APIError::ParseError(message)
            | APIError::FileError(message)
            | APIError::StreamError(message)
            | APIError::WebSocketError(message) => message.to_string(),
            APIError::UnknownError(status_code, message) => {
                format!("{status_code}: {message}")
            }
        }
    }
}

impl std::error::Error for APIError {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InvalidRequestError {
    pub code: String,
    pub message: String,
    pub param: Option<String>,
    pub r#type: String,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}

impl Display for InvalidRequestError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

#[cfg(feature = "realtime")]
impl From<reqwest_websocket::Error> for APIError {
    fn from(error: reqwest_websocket::Error) -> Self {
        APIError::WebSocketError(error.to_string())
    }
}
