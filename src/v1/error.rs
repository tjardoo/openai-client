use std::{fmt::{Display, Formatter, Result}, error::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum APIError {
    EndpointError(String),
    ParseError(String),
    FileError(String),
}

impl Error for APIError {}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            APIError::EndpointError(message) => write!(f, "{}", message),
            APIError::ParseError(message) => write!(f, "{}", message),
            APIError::FileError(message) => write!(f, "{}", message),
        }
    }
}
