use crate::v1::endpoints::assistants::assistants::Assistants;
use crate::v1::resources::shared::ListResponse;
use crate::v1::{
    error::APIError,
    helpers::format_response,
    resources::{assistant::run_step::RunStep, shared::ListParameters},
};

pub struct RunSteps<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Represents the steps (model and tool calls) taken during the run.
    pub fn run_steps(&self) -> RunSteps {
        RunSteps { assistant: self }
    }
}

impl RunSteps<'_> {
    /// Returns a list of run steps belonging to a run.
    pub async fn list(
        &self,
        thread_id: &str,
        run_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<RunStep>, APIError> {
        let response = self
            .assistant
            .client
            .get_with_query(&format!("/threads/{thread_id}/runs/{run_id}/steps"), &query)
            .await?;

        let response: ListResponse<RunStep> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a run step.
    pub async fn retrieve(
        &self,
        thread_id: &str,
        run_id: &str,
        run_step_id: &str,
    ) -> Result<RunStep, APIError> {
        let response = self
            .assistant
            .client
            .get(&format!(
                "/threads/{thread_id}/runs/{run_id}/steps/{run_step_id}"
            ))
            .await?;

        let response: RunStep = format_response(response)?;

        Ok(response)
    }
}
