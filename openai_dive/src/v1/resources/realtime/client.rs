use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::shared::Item;

fn default_type_value(value: &'static str) -> String {
    value.to_string()
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ConversationItemCreateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    pub id: Option<String>,
    /// The event type, must be "conversation.item.create".
    #[builder(default = "default_type_value(\"conversation.item.create\")")]
    pub r#type: String,
    /// The ID of the preceding item after which the new item will be inserted.
    pub previous_item_id: Option<String>,
    /// The item to add to the conversation.
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ResponseCreateBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ResponseCreate {
    /// Optional client-generated ID used to identify this event.
    pub event_id: Option<String>,
    /// The event type, must be "response.create".
    #[builder(default = "default_type_value(\"response.create\")")]
    pub r#type: String,
    // @todo
}
