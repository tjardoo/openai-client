use crate::v1::api::Client;

pub mod invites;
pub mod users;

pub struct Administration<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Programmatically manage your organization.
    pub fn administration(&self) -> Administration {
        Administration { client: self }
    }
}
