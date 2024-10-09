use client::*;
use server::{
    ConversationCreated, ConversationItemCreated, ConversationItemDeleted,
    ConversationItemInputAudioTranscriptionCompleted,
    ConversationItemInputAudioTranscriptionFailed, ConversationItemTruncated, Error,
    InputAudioBufferCleared, InputAudioBufferCommitted, InputAudioBufferSpeechStarted,
    InputAudioBufferSpeechStopped, RateLimitsUpdated, ResponseAudioDelta, ResponseAudioDone,
    ResponseAudioTranscriptDelta, ResponseAudioTranscriptDone, ResponseContentPartAdded,
    ResponseContentPartDone, ResponseCreated, ResponseDone, ResponseFunctionCallArgumentsDelta,
    ResponseFunctionCallArgumentsDone, ResponseOutputItemAdded, ResponseOutputItemDone,
    ResponseTextDelta, ResponseTextDone, SessionCreated, SessionUpdated,
};

use std::{collections::HashMap, fmt::Debug};

pub mod client;
pub mod resources;
pub mod server;

type DeserializeHashMap =
    HashMap<&'static str, fn(&str) -> Result<Box<dyn Debug>, serde_json::Error>>;

pub(crate) fn default_type_value(value: &'static str) -> String {
    value.to_string()
}

pub fn get_realtime_server_events_deserializers() -> DeserializeHashMap {
    let mut deserializers: DeserializeHashMap = HashMap::new();

    // add_client_deserializers(&mut deserializers);
    add_server_deserializers(&mut deserializers);

    deserializers
}

#[allow(dead_code)]
fn add_client_deserializers(deserializers: &mut DeserializeHashMap) {
    deserializers.insert("session.update", |text| {
        Ok(Box::new(serde_json::from_str::<SessionUpdate>(text)?))
    });
    deserializers.insert("input_audio_buffer.append", |text| {
        Ok(Box::new(serde_json::from_str::<InputAudioBufferAppend>(
            text,
        )?))
    });
    deserializers.insert("input_audio_buffer.commit", |text| {
        Ok(Box::new(serde_json::from_str::<InputAudioBufferCommit>(
            text,
        )?))
    });
    deserializers.insert("input_audio_buffer.clear", |text| {
        Ok(Box::new(serde_json::from_str::<InputAudioBufferClear>(
            text,
        )?))
    });
    deserializers.insert("conversation.item.create", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemCreate>(
            text,
        )?))
    });
    deserializers.insert("conversation.item.truncate", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemTruncate>(
            text,
        )?))
    });
    deserializers.insert("conversation.item.delete", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemDelete>(
            text,
        )?))
    });
    deserializers.insert("response.create", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseCreate>(text)?))
    });
    deserializers.insert("response.cancel", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseCancel>(text)?))
    });
}

fn add_server_deserializers(deserializers: &mut DeserializeHashMap) {
    deserializers.insert("error", |text| {
        Ok(Box::new(serde_json::from_str::<Error>(text)?))
    });
    deserializers.insert("session.created", |text| {
        Ok(Box::new(serde_json::from_str::<SessionCreated>(text)?))
    });
    deserializers.insert("session.updated", |text| {
        Ok(Box::new(serde_json::from_str::<SessionUpdated>(text)?))
    });
    deserializers.insert("conversation.created", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationCreated>(text)?))
    });
    deserializers.insert("input_audio_buffer.committed", |text| {
        Ok(Box::new(serde_json::from_str::<InputAudioBufferCommitted>(
            text,
        )?))
    });
    deserializers.insert("input_audio_buffer.cleared", |text| {
        Ok(Box::new(serde_json::from_str::<InputAudioBufferCleared>(
            text,
        )?))
    });
    deserializers.insert("input_audio_buffer.speech_started", |text| {
        Ok(Box::new(serde_json::from_str::<
            InputAudioBufferSpeechStarted,
        >(text)?))
    });
    deserializers.insert("input_audio_buffer.speech_stopped", |text| {
        Ok(Box::new(serde_json::from_str::<
            InputAudioBufferSpeechStopped,
        >(text)?))
    });
    deserializers.insert("conversation.item.created", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemCreated>(
            text,
        )?))
    });
    deserializers.insert(
        "conversation.item.input_audio_transcription.completed",
        |text| {
            Ok(Box::new(serde_json::from_str::<
                ConversationItemInputAudioTranscriptionCompleted,
            >(text)?))
        },
    );
    deserializers.insert(
        "conversation.item.input_audio_transcription.failed",
        |text| {
            Ok(Box::new(serde_json::from_str::<
                ConversationItemInputAudioTranscriptionFailed,
            >(text)?))
        },
    );
    deserializers.insert("conversation.item.truncated", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemTruncated>(
            text,
        )?))
    });
    deserializers.insert("conversation.item.deleted", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemDeleted>(
            text,
        )?))
    });
    deserializers.insert("response.created", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseCreated>(text)?))
    });
    deserializers.insert("response.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseDone>(text)?))
    });
    deserializers.insert("response.output_item.added", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseOutputItemAdded>(
            text,
        )?))
    });
    deserializers.insert("response.output_item.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseOutputItemDone>(
            text,
        )?))
    });
    deserializers.insert("response.content_part.added", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseContentPartAdded>(
            text,
        )?))
    });
    deserializers.insert("response.content_part.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseContentPartDone>(
            text,
        )?))
    });
    deserializers.insert("response.text.delta", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseTextDelta>(text)?))
    });
    deserializers.insert("response.text.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseTextDone>(text)?))
    });
    deserializers.insert("response.audio_transcript.delta", |text| {
        Ok(Box::new(serde_json::from_str::<
            ResponseAudioTranscriptDelta,
        >(text)?))
    });
    deserializers.insert("response.audio_transcript.done", |text| {
        Ok(Box::new(
            serde_json::from_str::<ResponseAudioTranscriptDone>(text)?,
        ))
    });
    deserializers.insert("response.audio.delta", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseAudioDelta>(text)?))
    });
    deserializers.insert("response.audio.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseAudioDone>(text)?))
    });
    deserializers.insert("response.function_call_arguments.delta", |text| {
        Ok(Box::new(serde_json::from_str::<
            ResponseFunctionCallArgumentsDelta,
        >(text)?))
    });
    deserializers.insert("response.function_call_arguments.done", |text| {
        Ok(Box::new(serde_json::from_str::<
            ResponseFunctionCallArgumentsDone,
        >(text)?))
    });
    deserializers.insert("rate_limits.updated", |text| {
        Ok(Box::new(serde_json::from_str::<RateLimitsUpdated>(text)?))
    });
}
