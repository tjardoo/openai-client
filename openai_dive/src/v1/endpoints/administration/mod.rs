use crate::v1::api::Client;

pub mod audit_logs;
pub mod invites;
pub mod project_api_keys;
pub mod project_rate_limits;
pub mod project_service_accounts;
pub mod project_users;
pub mod projects;
pub mod users;

pub struct Administration<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Programmatically manage your organization.
    pub fn administration(&self) -> Administration<'_> {
        Administration { client: self }
    }
}
