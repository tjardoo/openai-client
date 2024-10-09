use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Conversation {
    /// The unique ID of the conversation.
    pub id: String,
    /// The object type, must be "realtime.conversation".
    pub object: String,
}
