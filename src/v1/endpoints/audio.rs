use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::{file_from_bytes_to_form_part, file_from_disk_to_form_part};
use crate::v1::resources::audio::AudioSpeechParameters;
use crate::v1::resources::audio::AudioSpeechResponse;
#[cfg(feature = "stream")]
use crate::v1::resources::audio::AudioSpeechResponseChunkResponse;
use crate::v1::resources::audio::{
    AudioTranscriptionFile, AudioTranscriptionParameters, AudioTranslationParameters,
};
#[cfg(feature = "stream")]
use futures::Stream;
#[cfg(feature = "stream")]
use futures::StreamExt;
#[cfg(feature = "stream")]
use std::pin::Pin;

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

        let file = match parameters.file {
            AudioTranscriptionFile::Bytes(b) => file_from_bytes_to_form_part(b)?,
            AudioTranscriptionFile::File(f) => file_from_disk_to_form_part(f).await?,
        };

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

    #[cfg(feature = "stream")]
    /// Generates audio from the input text.
    pub async fn create_speech_stream(
        &self,
        parameters: AudioSpeechParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<AudioSpeechResponseChunkResponse, APIError>> + Send>>,
        APIError,
    > {
        use crate::v1::resources::audio::StreamAudioSpeechParameters;

        let stream_parameters = StreamAudioSpeechParameters {
            model: parameters.model,
            input: parameters.input,
            voice: parameters.voice,
            response_format: parameters.response_format,
            speed: parameters.speed,
            stream: true,
        };

        let stream = Box::pin(
            self.client
                .post_stream_raw("/audio/speech", &stream_parameters)
                .await
                .unwrap()
                .map(|item| item.map(|bytes| AudioSpeechResponseChunkResponse { bytes })),
        );

        Ok(stream)
    }
}
