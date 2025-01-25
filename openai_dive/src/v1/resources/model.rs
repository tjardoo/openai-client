use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: Option<u32>,
    /// The object type, which is always "model".
    pub object: String,
    /// The organization that owns the model.
    pub owned_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListModelResponse {
    /// The object type, which is always "list".
    pub object: String,
    /// A list of model objects.
    pub data: Vec<Model>,
}
