use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::usage::{
    AudioSpeechUsage, AudioTranscriptionUsage, CodeInterpreterSessionUsage, CompletionUsage,
    CostUsage, EmbeddingUsage, ImageUsage, ModerationUsage, UsageBucket, UsageParameters,
    VectorStoreUsage,
};

pub struct Usage<'a> {
    pub client: &'a Client,
}

impl Client {
    /// The Usage API provides detailed insights into your activity across the OpenAI API.
    pub fn usage(&self) -> Usage<'_> {
        Usage { client: self }
    }
}

impl Usage<'_> {
    /// Get completions usage details for the organization.
    pub async fn completions(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<CompletionUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/completions", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<CompletionUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get embeddings usage details for the organization.
    pub async fn embeddings(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<EmbeddingUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/embeddings", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<EmbeddingUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get moderations usage details for the organization.
    pub async fn moderations(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<ModerationUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/moderations", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<ModerationUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get images usage details for the organization.
    pub async fn images(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<ImageUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/images", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<ImageUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get audio speeches usage details for the organization.
    pub async fn audio_speeches(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<AudioSpeechUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/audio_speeches", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<AudioSpeechUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get audio transcriptions usage details for the organization.
    pub async fn audio_transcriptions(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<AudioTranscriptionUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/audio_transcriptions", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<AudioTranscriptionUsage>> =
            format_response(response)?;

        Ok(response)
    }

    /// The aggregated vector stores usage details of the specific time bucket.
    pub async fn vector_stores(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<VectorStoreUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/vector_stores", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<VectorStoreUsage>> = format_response(response)?;

        Ok(response)
    }

    /// Get code interpreter sessions usage details for the organization.
    pub async fn code_interpreter_sessions(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<CodeInterpreterSessionUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/code_interpreter_sessions", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<CodeInterpreterSessionUsage>> =
            format_response(response)?;

        Ok(response)
    }

    /// Get costs details for the organization.
    pub async fn costs(
        &self,
        parameters: UsageParameters,
    ) -> Result<ListResponse<UsageBucket<CostUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/costs", &parameters)
            .await?;

        let response: ListResponse<UsageBucket<CostUsage>> = format_response(response)?;

        Ok(response)
    }
}
