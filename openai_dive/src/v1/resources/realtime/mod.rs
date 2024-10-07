use crate::v1::resources::realtime::server::*;
use std::{collections::HashMap, fmt::Debug};

pub mod client;
pub mod server;
pub mod shared;

pub fn get_realtime_server_events_deserializers(
) -> HashMap<&'static str, fn(&str) -> Result<Box<dyn Debug>, serde_json::Error>> {
    let mut deserializers: HashMap<
        &'static str,
        fn(&str) -> Result<Box<dyn Debug>, serde_json::Error>,
    > = HashMap::new();

    deserializers.insert("error", |text| {
        Ok(Box::new(serde_json::from_str::<Error>(text)?))
    });
    deserializers.insert("session.created", |text| {
        Ok(Box::new(serde_json::from_str::<SessionCreated>(text)?))
    });
    deserializers.insert("session.updated", |text| {
        Ok(Box::new(serde_json::from_str::<SessionUpdated>(text)?))
    });
    deserializers.insert("response.created", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseCreated>(text)?))
    });
    deserializers.insert("response.done", |text| {
        Ok(Box::new(serde_json::from_str::<ResponseDone>(text)?))
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
    deserializers.insert("conversation.item.created", |text| {
        Ok(Box::new(serde_json::from_str::<ConversationItemCreated>(
            text,
        )?))
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
    deserializers.insert("rate_limits.updated", |text| {
        Ok(Box::new(serde_json::from_str::<RateLimitsUpdated>(text)?))
    });

    deserializers
}
