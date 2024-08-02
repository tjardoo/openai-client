use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    /// The object type, which is always organization.user
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The name of the user
    pub name: String,
    /// The email address of the user
    pub email: String,
    /// owner or reader
    pub role: UserRole,
    /// The Unix timestamp (in seconds) of when the user was added.
    pub added_at: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Owner,
    Reader,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "ModifyUserParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct ModifyUserParameters {
    /// The new role for the user
    pub role: UserRole,
}
