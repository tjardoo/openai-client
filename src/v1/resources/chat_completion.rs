use std::fmt::Display;
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Deserializer};
use crate::v1::models::OpenAIModel;
use crate::v1::resources::shared::{Usage, FinishReason};
use crate::v1::resources::shared::StopToken;

#[deprecated(since = "0.2.8")]
#[cfg(feature = "simple")]
#[derive(Serialize, Debug, Clone)]
pub struct SimpleChatCompletionParameters {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChatCompletionParameters {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallConfig>,
}

impl Default for ChatCompletionParameters {
    fn default() -> Self {
        ChatCompletionParameters {
            model: OpenAIModel::Gpt3_5Turbo.to_string(),
            messages: vec![
                ChatMessage {
                    role: Role::User,
                    content: "Hello!".to_string(),
                    name: None,
                    function_call: None,
                },
            ],
            temperature: None,
            top_p: None,
            n: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            functions: None,
            function_call: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: Role,
    #[serde(deserialize_with = "null_to_default")]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Role::System => "System",
                Role::User => "User",
                Role::Assistant => "Assistant",
                Role::Function => "Function",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    /// Function name
    pub name: String,

    /// Description of the function. 
    /// 
    /// Providing a good description lets the model know what the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSONSchema representation of function parameters as a JSON value
    /// 
    /// For simple functions, this can be constructed manually. For more complex use-cases, the [schemars](https://docs.rs/schemars) crate is recommended.
    /// 
    /// Resources:
    ///   - https://platform.openai.com/docs/guides/gpt/function-calling 
    ///   - JSONSchema: https://json-schema.org/ for more information.
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionCallConfig {
    /// Do not call any functions
    None,
    /// The model decides wether to call functions or not
    Auto,
    /// The model must call this function
    Force(ForceFunctionCall)
}

#[derive(Serialize, Debug, Clone)]

pub struct ForceFunctionCall {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    /// Name of the function to call
    pub name: String,

    /// JSON encoded arguments
    pub arguments: String,
}

fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let key = Option::<T>::deserialize(de)?;
    Ok(key.unwrap_or_default())
}