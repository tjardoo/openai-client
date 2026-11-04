use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::decision::{DecisionParameters, DecisionResponse};

pub struct Decisions<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Quickly answer classification or scoring questions about some input.
    pub fn decisions(&self) -> Decisions<'_> {
        Decisions { client: self }
    }
}

impl Decisions<'_> {
    /// Answers one or more predicate, choice or score questions about the input.
    pub async fn create(
        &self,
        parameters: DecisionParameters,
    ) -> Result<DecisionResponse, APIError> {
        let response = self.client.post("/decisions", &parameters, None).await?;

        let response: DecisionResponse = format_response(response.data)?;

        Ok(response)
    }
}
