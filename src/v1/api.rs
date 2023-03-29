use reqwest::multipart::{Form, Part};
use reqwest_eventsource::{RequestBuilderExt, EventSource, Event};
use serde::Serialize;
use tokio::fs::File;
use tokio_util::codec::{FramedRead, BytesCodec};
use super::error::APIError;
use futures::stream::StreamExt;

const OPENAI_API_V1_ENDPOINT: &str = "https://api.openai.com/v1";

pub struct Client {
    pub base_url: String,
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: OPENAI_API_V1_ENDPOINT.to_string(),
            api_key,
        }
    }

    pub async fn get(&self, path: &str) -> Result<String, APIError> {
        let client = reqwest::Client::new();

        let url = format!("{}{}", &self.base_url, path);

        let response = client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post<T: Serialize>(&self, path: &str, parameters: &T) -> Result<String, APIError> {
        let client = reqwest::Client::new();

        let url = format!("{}{}", &self.base_url, path);

        let response = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .json(&parameters)
            .send()
            .await
            .unwrap();

        if response.status().is_success() == false {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_with_form(&self, path: &str, form: Form) -> Result<String, APIError> {
        let client = reqwest::Client::new();

        let url = format!("{}{}", &self.base_url, path);

        let response = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "multipart/form-data")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .unwrap();

        if response.status().is_success() == false {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_stream<T>(
        &self,
        path: &str,
        parameters: &T
    ) -> Result<String, APIError>
    where
        T: Serialize,
    {
        let client = reqwest::Client::new();

        let url = format!("{}{}", &self.base_url, path);

        let event_source = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .json(&parameters)
            .eventsource()
            .unwrap();

        Client::process_stream(event_source).await;

        // @todo
        Ok("ok".to_string())
    }

    pub async fn process_stream(mut event_soure: EventSource) {
        // @todo
        // let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(event) = event_soure.next().await {
                match event {
                    Ok(item) => match item {
                        Event::Open => continue,
                        Event::Message(message) => {
                            // https://platform.openai.com/docs/api-reference/completions/create#completions/create-stream
                            if message.data == "[DONE]" {
                                break;
                            }

                            let _response = match serde_json::from_str::<String>(&message.data) {
                                Ok(result) => Ok(result),
                                Err(error) => Err(APIError::StreamError(error.to_string())),
                            };

                            // @todo
                        }
                    },
                    Err(_error) => {
                        break;
                    },
                }
            }

            event_soure.close();
        });
    }
}

pub async fn image_from_disk_to_form_part(path: String) -> Result<Part, APIError> {
    let file = File::open(path).await.map_err(|error| APIError::FileError(error.to_string()))?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = reqwest::Body::wrap_stream(stream);

    let image = reqwest::multipart::Part::stream(file_body)
        .file_name("image.png")
        .mime_str("application/octet-stream")
        .unwrap();

    Ok(image)
}
