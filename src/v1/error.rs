use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum APIError {
    EndpointError(String),
    ServerError(String),
    InvalidRequestError(String),
    ParseError(String),
    FileError(String),
    StreamError(String),
}

impl APIError {
    fn message(&self) -> &str {
        match self {
            APIError::EndpointError(message)
            | APIError::ServerError(message)
            | APIError::InvalidRequestError(message)
            | APIError::ParseError(message)
            | APIError::FileError(message)
            | APIError::StreamError(message) => message,
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
