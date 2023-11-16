use crate::v1::api::file_from_disk_to_form_part;
use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::audio::AudioSpeechParameters;
use crate::v1::resources::audio::AudioSpeechResponse;
use crate::v1::resources::audio::{AudioTranscriptionParameters, AudioTranslationParameters};

pub struct Audio<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Learn how to turn audio into text or text into audio.
    pub fn audio(&self) -> Audio {
        Audio { client: self }
    }
}

impl Audio<'_> {
    /// Generates audio from the input text.
    pub async fn create_speech(
        &self,
        parameters: AudioSpeechParameters,
    ) -> Result<AudioSpeechResponse, APIError> {
        let bytes = self.client.post_raw("/audio/speech", &parameters).await?;

        Ok(AudioSpeechResponse { bytes })
    }

    /// Transcribes audio into the input language.
    pub async fn create_transcription(
        &self,
        parameters: AudioTranscriptionParameters,
    ) -> Result<String, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let file = file_from_disk_to_form_part(parameters.file).await?;
        form = form.part("file", file);

        form = form.text("model", parameters.model);

        if let Some(prompt) = parameters.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(language) = parameters.language {
            form = form.text("language", language.to_string());
        }

        if let Some(response_format) = parameters.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(temperature) = parameters.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        let response = self
            .client
            .post_with_form("/audio/transcriptions", form)
            .await?;

        Ok(response)
    }

    /// Translates audio into English.
    pub async fn create_translation(
        &self,
        parameters: AudioTranslationParameters,
    ) -> Result<String, APIError> {
        let mut form = reqwest::multipart::Form::new();

        let file = file_from_disk_to_form_part(parameters.file).await?;
        form = form.part("file", file);

        form = form.text("model", parameters.model);

        if let Some(prompt) = parameters.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = parameters.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(temperature) = parameters.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        let response = self
            .client
            .post_with_form("/audio/translations", form)
            .await?;

        Ok(response)
    }
}
