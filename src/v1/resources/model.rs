use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    id: String,
    object: String,
    owned_by: String,
}
