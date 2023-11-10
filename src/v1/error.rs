use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize)]
pub enum APIError {
    EndpointError(String),
    ParseError(String),
    FileError(String),
    StreamError(String),
}

impl APIError {
    fn message(&self) -> &str {
        match self {
            APIError::EndpointError(message)
            | APIError::ParseError(message)
            | APIError::FileError(message)
            | APIError::StreamError(message) => message,
        }
    }
}

impl Error for APIError {}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}
