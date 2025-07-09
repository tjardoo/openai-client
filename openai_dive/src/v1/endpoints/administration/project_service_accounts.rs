use crate::v1::endpoints::administration::Administration;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::project_service_account::CreateProjectServiceAccountParameters;
use crate::v1::resources::administration::project_service_account::ProjectServiceAccount;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

pub struct ProjectServiceAccounts<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage service accounts within a project.
    pub fn project_service_accounts(&self) -> ProjectServiceAccounts {
        ProjectServiceAccounts {
            administration: self,
        }
    }
}

impl ProjectServiceAccounts<'_> {
    /// Returns a list of users in the project.
    pub async fn list(
        &self,
        project_id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<ProjectServiceAccount>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query(
                &format!("/organization/projects/{project_id}/service_accounts"),
                &query,
            )
            .await?;

        let response: ListResponse<ProjectServiceAccount> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a service account in the project.
    pub async fn retrieve(
        &self,
        project_id: &str,
        service_account_id: &str,
    ) -> Result<ProjectServiceAccount, APIError> {
        let response = self
            .administration
            .client
            .get(&format!(
                "/organization/projects/{project_id}/service_accounts/{service_account_id}"
            ))
            .await?;

        let response: ProjectServiceAccount = format_response(response)?;

        Ok(response)
    }

    /// Creates a new service account in the project.
    pub async fn create(
        &self,
        project_id: &str,
        parameters: CreateProjectServiceAccountParameters,
    ) -> Result<ProjectServiceAccount, APIError> {
        let response = self
            .administration
            .client
            .post(
                &format!("/organization/projects/{project_id}/service_accounts"),
                &parameters,
            )
            .await?;

        let response: ProjectServiceAccount = format_response(response.data)?;

        Ok(response)
    }

    /// Deletes a service account from the project.
    pub async fn delete(
        &self,
        project_id: &str,
        service_account_id: &str,
    ) -> Result<DeletedObject, APIError> {
        let response = self
            .administration
            .client
            .delete(&format!(
                "/organization/projects/{project_id}/service_accounts/{service_account_id}"
            ))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
