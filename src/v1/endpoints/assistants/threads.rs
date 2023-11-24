use crate::v1::endpoints::assistants::assistants::Assistants;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::thread::CreateThreadParameters;
use crate::v1::resources::assistant::thread::ModifyThreadParameters;
use crate::v1::resources::assistant::thread::Thread;
use crate::v1::resources::shared::DeletedObject;

pub struct Threads<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Create threads that assistants can interact with.
    pub fn threads(&self) -> Threads {
        Threads { assistant: self }
    }
}

impl Threads<'_> {
    /// Create a thread.
    pub async fn create(&self, parameters: CreateThreadParameters) -> Result<Thread, APIError> {
        let response = self
            .assistant
            .client
            .post(format!("/threads").as_str(), &parameters)
            .await?;

        let value = validate_request(response).await?;

        let thread_response: Thread = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(thread_response)
    }

    /// Retrieves a thread.
    pub async fn retrieve(&self, thread_id: &str) -> Result<Thread, APIError> {
        let response = self
            .assistant
            .client
            .get(format!("/threads/{thread_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let thread_response: Thread = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(thread_response)
    }

    /// Create threads that assistants can interact with.
    pub async fn modify(
        &self,
        thread_id: &str,
        parameters: ModifyThreadParameters,
    ) -> Result<Thread, APIError> {
        let response = self
            .assistant
            .client
            .post(format!("/threads/{thread_id}").as_str(), &parameters)
            .await?;

        let value = validate_request(response).await?;

        let thread_response: Thread = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(thread_response)
    }

    /// Delete a thread.
    pub async fn delete(&self, thread_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .assistant
            .client
            .delete(format!("/threads/{thread_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let deleted_object: DeletedObject = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(deleted_object)
    }
}
