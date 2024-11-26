use crate::v1::{
    endpoints::administration::Administration,
    error::APIError,
    helpers::format_response,
    resources::{
        administration::audit_log::{AuditLog, AuditLogParameters},
        shared::ListResponse,
    },
};

pub struct AuditLogs<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// List user actions and configuration changes within this organization.
    pub fn audit_logs(&self) -> AuditLogs {
        AuditLogs {
            administration: self,
        }
    }
}

impl AuditLogs<'_> {
    /// Logs of user actions and configuration changes within this organization. To log events, you must activate logging in the Organization Settings.
    pub async fn list(
        &self,
        query: Option<AuditLogParameters>,
    ) -> Result<ListResponse<AuditLog>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query("/organization/audit_logs", &query)
            .await?;

        let response: ListResponse<AuditLog> = format_response(response)?;

        Ok(response)
    }
}
