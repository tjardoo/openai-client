use serde::{Deserialize, Serialize};

use super::project_user::ProjectUserRole;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectApiKey {
    /// The object type, which is always organization.project.api_key.
    pub object: String,
    /// The redacted value of the API key
    pub redacted_value: String,
    /// The name of the API key.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the API key was created
    pub created_at: u32,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The owner of the API key.
    pub owner: ProjectApiKeyOwner,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectApiKeyOwner {
    /// The type of the owner.
    pub r#type: ProjectApiKeyOwnerType,
    /// The user that the API key belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ProjectApiKeyOwnerUser>,
    /// The service account that the API key belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ProjectApiKeyOwnerServiceAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApiKeyOwnerType {
    User,
    ServiceAccount,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectApiKeyOwnerUser {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the user.
    pub name: String,
    /// The email address of the user.
    pub email: String,
    /// The role of the user.
    pub role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the user was created.
    pub created_at: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectApiKeyOwnerServiceAccount {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the service account.
    pub name: String,
    /// The role of the service account.
    pub role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the service account was created
    pub created_at: u32,
}
