use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::usage::{CompletionUsage, CompletionUsageParameters, UsageData};

pub struct Usage<'a> {
    pub client: &'a Client,
}

impl Client {
    /// The Usage API provides detailed insights into your activity across the OpenAI API.
    pub fn usage(&self) -> Usage {
        Usage { client: self }
    }
}

impl Usage<'_> {
    /// Get completions usage details for the organization.
    pub async fn completions(
        &self,
        parameters: CompletionUsageParameters,
    ) -> Result<ListResponse<UsageData<CompletionUsage>>, APIError> {
        let response = self
            .client
            .get_with_query("/organization/usage/completions", &parameters)
            .await?;

        let response: ListResponse<UsageData<CompletionUsage>> = format_response(response)?;

        Ok(response)
    }
}
