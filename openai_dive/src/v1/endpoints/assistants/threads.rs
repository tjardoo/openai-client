use crate::v1::endpoints::assistants::assistants::Assistants;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
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
        let response = self.assistant.client.post("/threads", &parameters).await?;

        let response: Thread = format_response(response.data)?;

        Ok(response)
    }

    /// Retrieves a thread.
    pub async fn retrieve(&self, thread_id: &str) -> Result<Thread, APIError> {
        let response = self
            .assistant
            .client
            .get(&format!("/threads/{thread_id}"))
            .await?;

        let response: Thread = format_response(response)?;

        Ok(response)
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
            .post(&format!("/threads/{thread_id}"), &parameters)
            .await?;

        let response: Thread = format_response(response.data)?;

        Ok(response)
    }

    /// Delete a thread.
    pub async fn delete(&self, thread_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .assistant
            .client
            .delete(&format!("/threads/{thread_id}"))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
