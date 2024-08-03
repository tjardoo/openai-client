use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::project_user::ProjectUserRole;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectServiceAccount {
    /// The object type, which is always organization.project.service_account.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the service account.
    pub name: String,
    /// The role of the service account.
    pub role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the service account was created
    pub created_at: u32,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "CreateProjectServiceAccountParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct CreateProjectServiceAccountParameters {
    /// The name of the service account being created.
    pub name: String,
}
