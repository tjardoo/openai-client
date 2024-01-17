use crate::v1::helpers::is_beta_feature;
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
#[cfg(feature = "stream")]
use std::pin::Pin;

use super::resources::shared::ResponseWrapper;

const OPENAI_API_V1_ENDPOINT: &str = "https://api.openai.com/v1";

#[derive(Clone, Debug)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: OPENAI_API_V1_ENDPOINT.to_string(),
            api_key,
        }
    }

    pub fn build_request(
        &self,
        method: reqwest::Method,
        path: &str,
        content_type: &str,
    ) -> RequestBuilder {
        let url = format!("{}{}", &self.base_url, path);

        let mut request = self
            .http_client
            .request(method, &url)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .bearer_auth(&self.api_key);

        if is_beta_feature(path) {
            request = request.header("OpenAI-Beta", "assistants=v1");
        }

        request
    }

    pub async fn get(&self, path: &str) -> Result<String, APIError> {
        let response = self
            .build_request(Method::GET, path, "application/json")
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        let response_text = response.text().await.unwrap();

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(response_text)
    }

    pub async fn get_with_query<Q>(&self, path: &str, query: &Q) -> Result<String, APIError>
    where
        Q: Serialize,
    {
        let response = self
            .build_request(Method::GET, path, "application/json")
            .query(query)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        let response_text = response.text().await.unwrap();

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(response_text)
    }

    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        parameters: &T,
    ) -> Result<ResponseWrapper<String>, APIError> {
        let response = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        let header_map = response.headers().clone();

        let response_text = response.text().await.unwrap();
        let response_headers: Headers = header_map.into();

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(ResponseWrapper {
            data: response_text.to_string(),
            headers: response_headers,
        })
    }

    pub async fn delete(&self, path: &str) -> Result<String, APIError> {
        let response = self
            .build_request(Method::DELETE, path, "application/json")
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_with_form(&self, path: &str, form: Form) -> Result<String, APIError> {
        let response = self
            .build_request(
                Method::POST,
                path,
                format!("multipart/form-data; boundary={}", form.boundary()).as_str(),
            )
            .multipart(form)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_raw<T: Serialize>(
        &self,
        path: &str,
        parameters: &T,
    ) -> Result<Bytes, APIError> {
        let response = self
            .build_request(Method::POST, path, "application/json")
            .json(&parameters)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.bytes().await.unwrap())
    }

    #[cfg(feature = "stream")]
    pub async fn post_stream<I, O>(
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
    pub async fn post_stream_raw<I>(
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
    pub async fn process_stream<O>(
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
                                            error.to_string(),
                                            message.data
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
