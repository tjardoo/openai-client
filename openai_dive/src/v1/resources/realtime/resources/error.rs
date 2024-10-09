use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RealtimeError {
    /// The type of error.
    pub r#type: String,
    /// Error code.
    pub code: Option<String>,
    /// A human-readable error message.
    pub message: String,
    /// Parameter related to the error.
    pub param: Option<String>,
    /// The event_id of the client event that caused the error, if applicable.
    pub event_id: Option<String>,
}
