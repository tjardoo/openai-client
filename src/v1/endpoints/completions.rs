use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::completion::{CompletionParameters, CompletionResponse, CompletionStreamParameters};
use serde_json::Value;

pub struct Completions<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn completions(&self) -> Completions {
        Completions {
            client: self,
        }
    }
}

impl Completions<'_> {
    pub async fn create(&self, parameters: CompletionParameters) -> Result<CompletionResponse, APIError> {
        let response = self.client.post("/completions", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let completion_response: CompletionResponse = serde_json::from_value(value).map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(completion_response)
    }

    pub async fn create_stream(&self, parameters: CompletionParameters) -> Result<String, APIError> {
        let stream_parameters = CompletionStreamParameters {
            model: parameters.model,
            prompt: parameters.prompt,
            suffix: parameters.suffix,
            max_tokens: parameters.max_tokens,
            temperature: parameters.temperature,
            stream: true,
        };

        let response = self.client.post_stream("/completions", &stream_parameters).await?;

        Ok(response)
    }
}
