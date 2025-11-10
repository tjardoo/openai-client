use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::user::UserRole;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Invite {
    /// The object type, which is always organization.invite.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The email address of the individual to whom the invite was sent.
    pub email: String,
    /// The user role.
    pub role: UserRole,
    /// The status of the invite.
    pub status: InviteStatus,
    /// The Unix timestamp (in seconds) of when the invite was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<u32>,
    /// The Unix timestamp (in seconds) of when the invite expires.
    pub expires_at: u32,
    /// The Unix timestamp (in seconds) of when the invite was accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<u32>,
    /// The projects that were granted membership upon acceptance of the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectInvite>>,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "CreateInviteParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct CreateInviteParameters {
    /// Send an email to this address.
    pub email: String,
    /// The role for the user.
    pub role: UserRole,
    /// An array of projects to which membership is granted at the same time the org invite is accepted.
    /// If omitted, the user will be invited to the default project for compatibility with legacy behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectInvite>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    Accepted,
    Expired,
    Pending,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectInvite {
    /// Project's public ID
    pub id: String,
    /// Project membership role
    pub role: UserRole,
}
