use crate::v1::endpoints::administration::Administration;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::project_api_key::ProjectApiKey;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

pub struct ProjectApiKeys<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage API keys for a given project.
    pub fn project_api_keys(&self) -> ProjectApiKeys {
        ProjectApiKeys {
            administration: self,
        }
    }
}

impl ProjectApiKeys<'_> {
    /// Returns a list of API keys in the project.
    pub async fn list(
        &self,
        project_id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<ProjectApiKey>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query(
                &format!("/organization/projects/{}/api_keys", project_id),
                &query,
            )
            .await?;

        let response: ListResponse<ProjectApiKey> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves an API key in the project.
    pub async fn retrieve(
        &self,
        project_id: &str,
        api_key_id: &str,
    ) -> Result<ProjectApiKey, APIError> {
        let response = self
            .administration
            .client
            .get(&format!(
                "/organization/projects/{}/api_keys/{}",
                project_id, api_key_id
            ))
            .await?;

        let response: ProjectApiKey = format_response(response)?;

        Ok(response)
    }

    /// Deletes an API key from the project.
    pub async fn delete(
        &self,
        project_id: &str,
        api_key_id: &str,
    ) -> Result<DeletedObject, APIError> {
        let response = self
            .administration
            .client
            .delete(&format!(
                "/organization/projects/{}/api_keys/{}",
                project_id, api_key_id
            ))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
