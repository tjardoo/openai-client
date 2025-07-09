use crate::v1::endpoints::administration::Administration;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::project::CreateProjectParameters;
use crate::v1::resources::administration::project::ModifyProjectParameters;
use crate::v1::resources::administration::project::Project;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

pub struct Projects<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage the projects within an organization includes creation, updating, and archiving of projects.
    pub fn projects(&self) -> Projects {
        Projects {
            administration: self,
        }
    }
}

impl Projects<'_> {
    /// Lists all of the projects in the organization.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<Project>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query("/organization/projects", &query)
            .await?;

        let response: ListResponse<Project> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a project by their identifier.
    pub async fn retrieve(&self, project_id: &str) -> Result<Project, APIError> {
        let response = self
            .administration
            .client
            .get(&format!("/organization/projects/{project_id}"))
            .await?;

        let response: Project = format_response(response)?;

        Ok(response)
    }

    /// Create a new project in the organization.
    pub async fn create(&self, parameters: CreateProjectParameters) -> Result<Project, APIError> {
        let response = self
            .administration
            .client
            .post("/organization/projects", &parameters)
            .await?;

        let response: Project = format_response(response.data)?;

        Ok(response)
    }

    /// Modifies a project in the organization.
    pub async fn modify(
        &self,
        project_id: &str,
        parameters: ModifyProjectParameters,
    ) -> Result<Project, APIError> {
        let response = self
            .administration
            .client
            .post(&format!("/organization/projects/{project_id}"), &parameters)
            .await?;

        let response: Project = format_response(response.data)?;

        Ok(response)
    }

    /// Archives a project in the organization.
    pub async fn archive(&self, project_id: &str) -> Result<Project, APIError> {
        let response = self
            .administration
            .client
            .post(&format!("/organization/projects/{project_id}/archive"), &())
            .await?;

        let response: Project = format_response(response.data)?;

        Ok(response)
    }
}
