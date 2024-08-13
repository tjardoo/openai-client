use crate::v1::helpers::{check_status_code, is_beta_feature};
use crate::v1::{error::APIError, resources::shared::Headers};
use bytes::Bytes;
#[cfg(feature = "stream")]
use futures::{stream::StreamExt, Stream};
use reqwest::{multipart::Form, Method, RequestBuilder};
#[cfg(feature = "stream")]
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
#[cfg(feature = "stream")]
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
#[cfg(feature = "stream")]
use std::pin::Pin;

use super::resources::shared::ResponseWrapper;

const OPENAI_API_V1_ENDPOINT: &str = "https://api.openai.com/v1";

#[derive(Clone, Debug)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
    pub headers: Option<HashMap<String, String>>,
    pub organization: Option<String>,
    pub project: Option<String>,
}

impl Client {
    /// Create a new instance of the OpenAI client and set the API key.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    /// Create a new instance of the OpenAI client with a custom base URL and set the API key.
    pub fn new_with_base(base_url: &str, api_key: String) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key,
            ..Default::default()
        }
    }

    /// Create a new instance of the OpenAI client and set the API key from the environment variable `OPENAI_API_KEY`.
    pub fn new_from_env() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

        Self {
            api_key,
            ..Default::default()
        }
    }

    /// Set the organization header for the OpenAI client.
    pub fn set_organization(&mut self, organization: &str) -> &mut Self {
        self.organization = Some(organization.to_string());

        self
    }

    /// Set the project header for the OpenAI client.
    pub fn set_project(&mut self, project: &str) -> &mut Self {
        self.project = Some(project.to_string());

        self
    }

    /// Add a custom header to the OpenAI client.
    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());

        self
    }

    fn build_request(
        &self,
        method: reqwest::Method,
        path: &str,
        content_type: &str,
    ) -> RequestBuilder {
        let url = format!("{}{}", &self.base_url, path);

        let mut request = self
            .http_client
            .request(method, url)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .bearer_auth(&self.api_key);

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if let Some(organization) = &self.organization {
            request = request.header("OpenAI-Organization", organization);
        }

        if let Some(project) = &self.project {
            request = request.header("OpenAI-Project", project);
        }

        if is_beta_feature(path) {
            request = request.header("OpenAI-Beta", "assistants=v2");
        }

        request
    }

    pub(crate) async fn get(&self, path: &str) -> Result<String, APIError> {
        let result = self
            .build_request(Method::GET, path, "application/json")
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        let response_text = response
            .text()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(response_text)
    }

    pub(crate) async fn get_with_query<Q>(&self, path: &str, query: &Q) -> Result<String, APIError>
    where
        Q: Serialize,
    {
        let result = self
            .build_request(Method::GET, path, "application/json")
            .query(query)
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        let response_text = response
            .text()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(response_text)
    }

    pub(crate) async fn post<T: Serialize>(
        &self,
        path: &str,
        parameters: &T,
    ) -> Result<ResponseWrapper<String>, APIError> {
        let result = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        let header_map = response.headers().clone();

        let response_text = response
            .text()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))?;
        let response_headers: Headers = header_map.into();

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(ResponseWrapper {
            data: response_text.to_string(),
            headers: response_headers,
        })
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<String, APIError> {
        let result = self
            .build_request(Method::DELETE, path, "application/json")
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        response
            .text()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))
    }

    pub(crate) async fn post_with_form(&self, path: &str, form: Form) -> Result<String, APIError> {
        #[cfg(not(target_arch = "wasm32"))]
        let content_type = format!("multipart/form-data; boundary={}", form.boundary());
        #[cfg(target_arch = "wasm32")]
        let content_type = format!("multipart/form-data");

        let result = self
            .build_request(Method::POST, path, &content_type)
            .multipart(form)
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        response
            .text()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))
    }

    pub(crate) async fn post_raw<T: Serialize>(
        &self,
        path: &str,
        parameters: &T,
    ) -> Result<Bytes, APIError> {
        let result = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .send()
            .await;

        let response = match check_status_code(result).await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        response
            .bytes()
            .await
            .map_err(|error| APIError::ParseError(error.to_string()))
    }

    #[cfg(feature = "stream")]
    pub(crate) async fn post_stream<I, O>(
        &self,
        path: &str,
        parameters: &I,
    ) -> Pin<Box<dyn Stream<Item = Result<O, APIError>> + Send>>
    where
        I: Serialize,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let event_source = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .eventsource()
            .unwrap();

        Client::process_stream::<O>(event_source).await
    }

    #[cfg(feature = "stream")]
    pub(crate) async fn post_stream_raw<I>(
        &self,
        path: &str,
        parameters: &I,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, APIError>> + Send>>, APIError>
    where
        I: Serialize,
    {
        let stream = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .send()
            .await
            .unwrap()
            .bytes_stream()
            .map(|item| item.map_err(|error| APIError::StreamError(error.to_string())));

        Ok(Box::pin(stream)
            as Pin<
                Box<dyn Stream<Item = Result<Bytes, APIError>> + Send>,
            >)
    }

    #[cfg(feature = "stream")]
    pub(crate) async fn process_stream<O>(
        mut event_soure: EventSource,
    ) -> Pin<Box<dyn Stream<Item = Result<O, APIError>> + Send>>
    where
        O: DeserializeOwned + Send + 'static,
    {
        use super::error::InvalidRequestError;

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(event_result) = event_soure.next().await {
                match event_result {
                    Ok(event) => match event {
                        Event::Open => continue,
                        Event::Message(message) => {
                            if message.data == "[DONE]" {
                                break;
                            }

                            let response = match serde_json::from_str::<O>(&message.data) {
                                Ok(result) => Ok(result),
                                Err(error) => {
                                    match serde_json::from_str::<InvalidRequestError>(&message.data)
                                    {
                                        Ok(invalid_request_error) => Err(APIError::StreamError(
                                            invalid_request_error.to_string(),
                                        )),
                                        Err(_) => Err(APIError::StreamError(format!(
                                            "{} {}",
                                            error, message.data
                                        ))),
                                    }
                                }
                            };

                            if let Err(_error) = tx.send(response) {
                                break;
                            }
                        }
                    },
                    Err(error) => {
                        if let Err(_error) = tx.send(Err(APIError::StreamError(error.to_string())))
                        {
                            break;
                        }
                    }
                }
            }

            event_soure.close();
        });

        Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            http_client: reqwest::Client::new(),
            base_url: OPENAI_API_V1_ENDPOINT.to_string(),
            api_key: "".to_string(),
            headers: None,
            organization: None,
            project: None,
        }
    }
}
