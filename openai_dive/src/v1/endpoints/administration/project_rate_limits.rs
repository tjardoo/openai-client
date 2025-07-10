use crate::v1::{
    endpoints::administration::Administration,
    error::APIError,
    helpers::format_response,
    resources::{
        administration::project_rate_limit::{ModifyProjectRateLimitParameters, ProjectRateLimit},
        shared::{ListParameters, ListResponse},
    },
};

pub struct ProjectRateLimits<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage rate limits per model for projects. Rate limits may be configured to be equal to or lower than the organization's rate limits.
    pub fn project_rate_limits(&self) -> ProjectRateLimits {
        ProjectRateLimits {
            administration: self,
        }
    }
}

impl ProjectRateLimits<'_> {
    /// Returns the rate limits per model for a project.
    pub async fn list(
        &self,
        project_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<ProjectRateLimit>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query(
                &format!("/organization/projects/{project_id}/rate_limits"),
                &query,
            )
            .await?;

        let response: ListResponse<ProjectRateLimit> = format_response(response)?;

        Ok(response)
    }

    /// Updates a project rate limit.
    pub async fn modify(
        &self,
        project_id: &str,
        rate_limit_id: &str,
        parameters: ModifyProjectRateLimitParameters,
    ) -> Result<ProjectRateLimit, APIError> {
        let response = self
            .administration
            .client
            .post(
                &format!("/organization/projects/{project_id}/rate_limits/{rate_limit_id}"),
                &parameters,
                None,
            )
            .await?;

        let response: ProjectRateLimit = format_response(response.data)?;

        Ok(response)
    }
}
