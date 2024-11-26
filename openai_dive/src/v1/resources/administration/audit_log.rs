use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLog {
    /// The object type, which is always "organization.audit_log".
    pub object: String,
    /// The ID of this log.
    pub id: String,
    /// The event type.
    pub r#type: String,
    /// The Unix timestamp (in seconds) of the event.
    pub effective_at: u32,
    /// The project that the action was scoped to. Absent for actions not scoped to projects.
    pub project: Option<AuditLogProject>,
    /// The actor who performed the audit logged action.
    pub actor: Option<AuditLogActor>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogProject {
    /// The project ID.
    pub id: String,
    /// The project title.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogActor {
    /// The type of actor. Is either 'session' or 'api_key'.
    pub r#type: String,
    /// The session in which the audit logged action was performed.
    pub session: Option<AuditLogActorSession>,
    /// The API Key used to perform the audit logged action.
    pub api_key: Option<AuditLogActorApiKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogActorSession {
    /// The user who performed the audit logged action.
    pub user: AuditLogUser,
    /// The IP address from which the action was performed.
    pub ip_address: String,
    /// The user agent from which the action was performed.
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogUser {
    /// The user id.
    pub id: String,
    /// The user email.
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogServiceAccount {
    /// The service account id.
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogActorApiKey {
    /// The tracking id of the API key.
    pub id: String,
    /// The type of API key. Can be either 'user' or 'service_account'.
    pub r#type: String,
    /// The user who performed the audit logged action.
    pub user: Option<AuditLogUser>,
    /// The service account that performed the audit logged action.
    pub service_account: Option<AuditLogServiceAccount>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "AuditLogParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct AuditLogParameters {
    /// Return only events whose effective_at (Unix seconds) is in this range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<AuditLogParametersEffectiveAt>,
    /// Return only events for these projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Return only events with a type in one of these values. For example, 'project.created'.
    #[serde(rename = "event_types[]")]
    pub event_types: Option<Vec<String>>,
    /// Return only events performed by these actors. Can be a user ID, a service account ID, or an api key tracking ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<String>>,
    /// Return only events performed by users with these emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_emails: Option<Vec<String>>,
    /// Return only events performed on these targets. For example, a project ID updated.
    pub resource_ids: Option<Vec<String>>,
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. after is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. before is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuditLogParametersEffectiveAt {
    /// Return only events whose effective_at (Unix seconds) is greater than this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<u32>,
    /// Return only events whose effective_at (Unix seconds) is greater than or equal to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<u32>,
    /// Return only events whose effective_at (Unix seconds) is less than this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<u32>,
    /// Return only events whose effective_at (Unix seconds) is less than or equal to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<u32>,
}
