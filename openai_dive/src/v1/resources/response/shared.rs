#[cfg(feature = "stream")]
use crate::v1::error::APIError;
#[cfg(feature = "stream")]
use crate::v1::resources::response::response::ResponseStreamEvent;
use crate::v1::resources::{
    image::{BackgroundStyle, ImageSize},
    shared::{InputTokensDetails, OutputTokensDetails, WebSearchContextSize},
};
#[cfg(feature = "stream")]
use futures::Stream;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "stream")]
use std::pin::Pin;

#[cfg(feature = "stream")]
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<ResponseStreamEvent, APIError>> + Send>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ResponseToolChoice {
    None,
    Auto,
    Required,
    Function(String),
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUsage {
    /// The number of input tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u32>,
    /// A detailed breakdown of the input tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens_details: Option<InputTokensDetails>,
    /// The number of output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u32>,
    /// A detailed breakdown of the output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<OutputTokensDetails>,
    /// Number of tokens in the entire response.
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseTool {
    Function {
        name: String,
        description: Option<String>,
        parameters: serde_json::Value,
        strict: bool,
    },
    FileSearch {
        vector_store_ids: Vec<String>,
        filters: Option<FileSearchFilters>,
        max_num_results: Option<u32>,
        ranking_options: Option<FileSearchRankingOption>,
    },
    #[serde(rename = "web_search_preview")]
    WebSearch {
        search_context_size: Option<WebSearchContextSize>,
        user_location: Option<WebSearchUserLocation>,
    },
    #[serde(rename = "mcp")]
    Mcp {
        // A label for this MCP server, used to identify it in tool calls.
        server_label: String,
        // The URL for the MCP server. One of server_url or connector_id must be provided.
        server_url: Option<String>,
        // Optional description of the MCP server, used to provide more context.
        server_description: Option<String>,
        // A string array of allowed tool names
        allowed_tools: Option<Vec<String>>,
        // An OAuth access token that can be used with a remote MCP server, either with a custom MCP server URL or a service connector. Your application must handle the OAuth authorization flow and provide the token here.
        authorization: String,
        // Identifier for service connectors. One of server_url or connector_id must be provided.
        connector_id: Option<String>,
        // Optional HTTP headers to send to the MCP server. Use for authentication or other purposes.
        headers: Option<HashMap<String, String>>,
        // Specify which of the MCP server's tools require approval.
        require_approval: Option<McpToolApproval>,
    },
    #[serde(rename = "image_generation")]
    ImageGeneration {
        // Background type for the generated image. One of transparent, opaque, or auto.
        background: Option<BackgroundStyle>,
        // Control how much effort the model will exert to match the style and features, especially facial features, of input images.
        input_fidelity: Option<String>,
        // Optional mask for inpainting. Contains image_url (string, optional) and file_id (string, optional).
        input_image_mask: Option<InputImageMask>,
        // The image generation model to use.
        model: Option<String>,
        // Moderation level for the generated image.
        moderation: Option<String>,
        // Compression level for the output image.
        output_compression: Option<u32>,
        // The output format of the generated image. One of png, webp, or jpeg.
        output_format: Option<String>,
        // Number of partial images to generate in streaming mode, from 0 (default value) to 3.
        partial_images: Option<u32>,
        // The quality of the generated image. One of low, medium, high, or auto.
        quality: Option<String>,
        // The size of the generated image. One of 1024x1024, 1024x1536, 1536x1024, or auto.
        size: Option<ImageSize>,
    },
    #[serde(rename = "computer_use_preview")]
    ComputerUse {
        display_height: Option<u64>,
        display_width: Option<u64>,
        environment: Option<ComputerUseEnvironment>,
    },
    #[serde(rename = "code_interpreter")]
    CodeInterpreter { container: String },
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "apply_patch")]
    ApplyPatch,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        description: Option<String>,
        format: Option<CustomToolFormat>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputImageMask {
    pub image_url: Option<String>,
    pub file_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CustomToolFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "grammar")]
    Grammar { definition: String, syntax: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum McpToolApproval {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSearchRankingOption {
    pub ranker: String,
    pub score_threshold: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebSearchUserLocation {
    pub r#type: UserLocationType,
    pub city: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserLocationType {
    Approximate,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema {
        schema: serde_json::Value,
        name: String,
        description: String,
        strict: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TruncationStrategy {
    Auto,
    Disabled,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComputerUseEnvironment {
    Browser,
    Mac,
    Ubuntu,
    Windows,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FileSearchFilters {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ComparisonFilter {
    pub key: String,
    pub r#type: String,
    pub value: ComparisonFilterValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CompoundFilter {
    pub filters: Vec<FileSearchFilters>,
    pub r#type: CompoundFilterType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompoundFilterType {
    And,
    Or,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation { file_id: String, index: u64 },
    #[serde(rename = "url_citation")]
    URLCitation {
        title: String,
        url: String,
        start_index: u64,
        end_index: u64,
    },
    #[serde(rename = "file_path")]
    FilePath { file_id: String, index: u64 },
}

impl Serialize for ResponseToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::Required => serializer.serialize_str("required"),
            Self::FileSearch => {
                let mut fn_struct = serializer.serialize_struct("Function", 1)?;
                fn_struct.serialize_field("type", "file_search")?;
                fn_struct.end()
            }
            Self::WebSearchPreview => {
                let mut fn_struct = serializer.serialize_struct("Function", 1)?;
                fn_struct.serialize_field("type", "web_search_preview")?;
                fn_struct.end()
            }
            Self::ComputerUsePreview => {
                let mut fn_struct = serializer.serialize_struct("Function", 1)?;
                fn_struct.serialize_field("type", "computer_use_preview")?;
                fn_struct.end()
            }
            Self::Function(name) => {
                let mut fn_struct = serializer.serialize_struct("Function", 2)?;
                fn_struct.serialize_field("name", name)?;
                fn_struct.serialize_field("type", "function")?;
                fn_struct.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ResponseToolChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ToolChoiceVisitor;

        impl<'de> Visitor<'de> for ToolChoiceVisitor {
            type Value = ResponseToolChoice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or struct")
            }

            fn visit_str<E>(self, value: &str) -> Result<ResponseToolChoice, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "none" => Ok(ResponseToolChoice::None),
                    "auto" => Ok(ResponseToolChoice::Auto),
                    "required" => Ok(ResponseToolChoice::Required),
                    _ => Err(serde::de::Error::unknown_variant(
                        value,
                        &["none", "auto", "required"],
                    )),
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut record = HashMap::<String, String>::new();

                while let Some((key, value)) = map.next_entry()? {
                    record.insert(key, value);
                }

                let Some(r#type) = record.get("type") else {
                    return Err(serde::de::Error::missing_field("type"));
                };

                match r#type.as_str() {
                    "file_search" => Ok(ResponseToolChoice::FileSearch),
                    "web_search_preview" => Ok(ResponseToolChoice::WebSearchPreview),
                    "computer_use_preview" => Ok(ResponseToolChoice::ComputerUsePreview),
                    "function" => {
                        let Some(name) = record.get("name") else {
                            return Err(serde::de::Error::missing_field("name"));
                        };
                        Ok(ResponseToolChoice::Function(name.clone()))
                    }
                    _ => Err(serde::de::Error::unknown_variant(
                        r#type.as_str(),
                        &[
                            "file_search",
                            "web_search_preview",
                            "computer_use_preview",
                            "function",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_any(ToolChoiceVisitor {})
    }
}
