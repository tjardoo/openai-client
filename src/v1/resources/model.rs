use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    id: String,
    object: String,
    owned_by: String,
}
