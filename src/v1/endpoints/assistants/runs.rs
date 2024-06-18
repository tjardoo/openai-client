use crate::v1::endpoints::assistants::assistants::Assistants;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::assistant::assistant::ToolOutputsParameters;
use crate::v1::resources::assistant::run::CreateRunParameters;
use crate::v1::resources::assistant::run::CreateThreadAndRunParameters;
use crate::v1::resources::assistant::run::ModifyRunParameters;
use crate::v1::resources::assistant::run::Run;
use crate::v1::resources::shared::ListParameters;
use crate::v1::resources::shared::ListResponse;

pub struct Runs<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Represents an execution run on a thread.
    pub fn runs(&self) -> Runs {
        Runs { assistant: self }
    }
}

impl Runs<'_> {
    /// Create a run.
    pub async fn create(
        &self,
        thread_id: &str,
        parameters: CreateRunParameters,
    ) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .post(format!("/threads/{thread_id}/runs").as_str(), &parameters)
            .await?;

        let run_response: Run = format_response(response.data)?;

        Ok(run_response)
    }

    /// Create a thread and run it in one request.
    pub async fn create_thread_and_run(
        &self,
        parameters: CreateThreadAndRunParameters,
    ) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .post(format!("/threads/runs").as_str(), &parameters)
            .await?;

        let run_response: Run = format_response(response.data)?;

        Ok(run_response)
    }

    /// Retrieves a run.
    pub async fn retrieve(&self, thread_id: &str, run_id: &str) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .get(format!("/threads/{thread_id}/runs/{run_id}").as_str())
            .await?;

        let run_response: Run = format_response(response)?;

        Ok(run_response)
    }

    /// Modifies a run.
    pub async fn modify(
        &self,
        thread_id: &str,
        run_id: &str,
        parameters: ModifyRunParameters,
    ) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .post(
                format!("/threads/{thread_id}/runs/{run_id}").as_str(),
                &parameters,
            )
            .await?;

        let run_response: Run = format_response(response.data)?;

        Ok(run_response)
    }

    /// Returns a list of runs belonging to a thread.
    pub async fn list(
        &self,
        thread_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListResponse<Run>, APIError> {
        let response = self
            .assistant
            .client
            .get_with_query(format!("/threads/{thread_id}/runs").as_str(), &query)
            .await?;

        let list_runs_response: ListResponse<Run> = format_response(response)?;

        Ok(list_runs_response)
    }

    /// Cancels a run that is 'in_progress'.
    pub async fn cancel(&self, thread_id: &str, run_id: &str) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .post(
                format!("/threads/{thread_id}/runs/{run_id}/cancel").as_str(),
                &serde_json::json!({}),
            )
            .await?;

        let run_response: Run = format_response(response.data)?;

        Ok(run_response)
    }

    /// When a run has the status: 'requires_action' and required_action.type is 'submit_tool_outputs',
    /// this endpoint can be used to submit the outputs from the tool calls once they're all completed.
    /// All outputs must be submitted in a single request.
    pub async fn submit_tool_outputs(
        &self,
        thread_id: &str,
        run_id: &str,
        parameters: ToolOutputsParameters,
    ) -> Result<Run, APIError> {
        let response = self
            .assistant
            .client
            .post(
                format!("/threads/{thread_id}/runs/{run_id}/submit_tool_outputs").as_str(),
                &parameters,
            )
            .await?;

        let run_response: Run = format_response(response.data)?;

        Ok(run_response)
    }
}
