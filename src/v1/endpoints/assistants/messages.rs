use crate::v1::endpoints::assistants::assistants::Assistants;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::assistant::message::CreateMessageParameters;
use crate::v1::resources::assistant::message::ListMessageFilesResponse;
use crate::v1::resources::assistant::message::ListMessagesResponse;
use crate::v1::resources::assistant::message::Message;
use crate::v1::resources::assistant::message::MessageFile;
use crate::v1::resources::assistant::message::ModifyMessageParameters;
use crate::v1::resources::shared::ListParameters;

pub struct Messages<'a> {
    pub assistant: &'a Assistants<'a>,
}

impl Assistants<'_> {
    /// Create messages within threads
    pub fn messages(&self) -> Messages {
        Messages { assistant: self }
    }
}

impl Messages<'_> {
    /// Create a message.
    pub async fn create(
        &self,
        thread_id: &str,
        parameters: CreateMessageParameters,
    ) -> Result<Message, APIError> {
        let response = self
            .assistant
            .client
            .post(
                format!("/threads/{thread_id}/messages").as_str(),
                &parameters,
            )
            .await?;

        let value = validate_request(response).await?;

        let message_response: Message = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(message_response)
    }

    /// Retrieve a message.
    pub async fn retrieve(&self, thread_id: &str, message_id: &str) -> Result<Message, APIError> {
        let response = self
            .assistant
            .client
            .get(format!("/threads/{thread_id}/messages/{message_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let message_response: Message = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(message_response)
    }

    /// Modifies a message.
    pub async fn modify(
        &self,
        thread_id: &str,
        message_id: &str,
        parameters: ModifyMessageParameters,
    ) -> Result<Message, APIError> {
        let response = self
            .assistant
            .client
            .post(
                format!("/threads/{thread_id}/messages/{message_id}").as_str(),
                &parameters,
            )
            .await?;

        let value = validate_request(response).await?;

        let message_response: Message = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(message_response)
    }

    /// Returns a list of messages for a given thread.
    pub async fn list(
        &self,
        thread_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListMessagesResponse, APIError> {
        let response = self
            .assistant
            .client
            .get_with_query(format!("/threads/{thread_id}/messages").as_str(), &query)
            .await?;

        let value = validate_request(response).await?;

        let list_messages_response: ListMessagesResponse = serde_json::from_value(value.clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_messages_response)
    }

    /// Retrieves a message file.
    pub async fn retrieve_file(
        &self,
        thread_id: &str,
        message_id: &str,
        file_id: &str,
    ) -> Result<MessageFile, APIError> {
        let response = self
            .assistant
            .client
            .get(format!("/threads/{thread_id}/messages/{message_id}/files/{file_id}").as_str())
            .await?;

        let value = validate_request(response).await?;

        let message_file_response: MessageFile = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(message_file_response)
    }

    /// Returns a list of message files.
    pub async fn list_files(
        &self,
        thread_id: &str,
        message_id: &str,
        query: Option<ListParameters>,
    ) -> Result<ListMessageFilesResponse, APIError> {
        let response = self
            .assistant
            .client
            .get_with_query(
                format!("/threads/{thread_id}/messages/{message_id}/files").as_str(),
                &query,
            )
            .await?;

        let value = validate_request(response).await?;

        let list_message_files_response: ListMessageFilesResponse =
            serde_json::from_value(value.clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(list_message_files_response)
    }
}
