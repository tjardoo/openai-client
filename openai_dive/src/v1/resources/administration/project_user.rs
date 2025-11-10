use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectUser {
    /// The object type, which is always organization.project.user.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the user.
    pub name: String,
    /// The email address of the user.
    pub email: String,
    /// The role of the user.
    pub role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the user was added.
    pub added_at: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectUserRole {
    Owner,
    Member,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "CreateProjectUserParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct CreateProjectUserParameters {
    /// The ID of the user.
    pub user: String,
    /// The role of the user.
    pub role: ProjectUserRole,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "ModifyProjectUserParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct ModifyProjectUserParameters {
    /// The new role of the user.
    pub role: ProjectUserRole,
}
