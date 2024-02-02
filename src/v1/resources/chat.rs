use crate::v1::models::Gpt4Engine;
use crate::v1::resources::shared::StopToken;
use crate::v1::resources::shared::{FinishReason, Usage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if n is greater than 1.
    pub choices: Vec<ChatCompletionChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u32,
    /// The model used for the chat completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// The object type, which is always chat.completion.
    pub object: String,
    /// Usage statistics for the completion request.
    pub usage: Option<Usage>,
}

#[cfg(feature = "stream")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionChunkResponse {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if n is greater than 1.
    pub choices: Vec<ChatCompletionChunkChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: u32,
    /// The model to generate the completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// The object type, which is always chat.completion.chunk.
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionParameters {
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<ChatMessage>,
    /// ID of the model to use.
    pub model: String,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// Whether to return log probabilities of the output tokens or not.
    /// If true, returns the log probabilities of each output token returned in the 'content' of 'message'.
    /// This option is currently not available on the 'gpt-4-vision-preview' model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// An integer between 0 and 5 specifying the number of most likely tokens to return at each token position,
    /// each with an associated log probability. 'logprobs' must be set to 'true' if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// The maximum number of tokens to generate in the chat completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// An object specifying the format that the model must output.
    /// Setting to { "type": "json_object" } enables JSON mode, which guarantees the message the model generates is valid JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ChatCompletionResponseFormat>,
    /// This feature is in Beta. If specified, our system will make a best effort to sample deterministically,
    /// such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    /// Controls which (if any) function is called by the model. none means the model will not call a function and instead generates a message.
    /// 'auto' means the model can pick between generating a message or calling a function.
    /// Specifying a particular function via {"type: "function", "function": {"name": "my_function"}} forces the model to call that function.
    /// 'none' is the default when no functions are present. 'auto' is the default if functions are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoice>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionToolChoiceFunction {
    /// The type of the tool. Currently, only 'function' is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ChatCompletionToolType>,
    /// Name of the function.
    pub function: ChatCompletionToolChoiceFunctionName,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionToolChoiceFunctionName {
    /// Name of the function.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionFunction {
    /// Name of the function.
    pub name: String,
    /// Optional description of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the function takes. The model will generate JSON inputs for these parameters.
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionResponseFormat {
    /// The type of response format. Currently, only 'json_object' and 'text' are supported.
    pub r#type: ChatCompletionResponseFormatType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionTool {
    /// The type of the tool. Currently, only 'function' is supported.
    pub r#type: ChatCompletionToolType,
    /// The name of the function to call.
    pub function: ChatCompletionFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    /// The role of the author of this message.
    pub role: Role,
    /// The content of the message.
    pub content: ChatMessageContent,
    /// The tool calls generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When responding to a tool call; provide the id of the tool call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[cfg(feature = "stream")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeltaChatMessage {
    /// The role of the author of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// The contents of the chunk message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The tool calls generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<DeltaToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolCall {
    /// The index of the tool call in the list of tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    /// The ID of the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the tool. Currently, only 'function' is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The function that the model called.
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeltaToolCall {
    /// The index of the tool call in the list of tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    /// /// The ID of the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the tool. Currently, only 'function' is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The function that the model called.
    pub function: DeltaFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Function {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format.
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeltaFunction {
    /// The name of the function to call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The arguments to call the function with, as generated by the model in JSON format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionChoice {
    /// The index of the choice in the list of choices.
    pub index: Option<u32>,
    /// A chat completion message generated by the model.
    pub message: ChatMessage,
    /// The reason the model stopped generating tokens.
    pub finish_reason: Option<FinishReason>,
}

#[cfg(feature = "stream")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionChunkChoice {
    /// The index of the choice in the list of choices.
    pub index: Option<u32>,
    /// A chat completion delta generated by streamed model responses.
    pub delta: DeltaChatMessage,
    /// The reason the model stopped generating tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
}

#[cfg(feature = "stream")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamChatCompletionParameters {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ChatCompletionResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUrl {
    /// The type of the content part.
    pub r#type: String,
    /// The text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The image URL.
    pub image_url: ImageUrlType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUrlType {
    /// Either a URL of the image or the base64 encoded image data.
    pub url: String,
    /// Specifies the detail level of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageUrlDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageUrlDetail {
    Auto,
    High,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatMessageContent {
    Text(String),
    ImageUrl(Vec<ImageUrl>),
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChatCompletionResponseFormatType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolType {
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionToolChoice {
    None,
    Auto,
    ChatCompletionToolChoiceFunction(ChatCompletionToolChoiceFunction),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
    Tool,
}

impl Default for ChatCompletionParameters {
    fn default() -> Self {
        ChatCompletionParameters {
            messages: vec![ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text("Hello, World!".to_string()),
                tool_calls: None,
                name: None,
                tool_call_id: None,
            }],
            model: Gpt4Engine::Gpt4.to_string(),
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            stop: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            user: None,
        }
    }
}

impl Default for ChatMessage {
    fn default() -> Self {
        ChatMessage {
            role: Role::User,
            content: ChatMessageContent::Text("Hello, World!".to_string()),
            tool_calls: None,
            name: None,
            tool_call_id: None,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Function => "function",
                Role::Tool => "tool",
            }
        )
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            "function" => Ok(Role::Function),
            "tool" => Ok(Role::Tool),
            _ => Err(format!("{} is not a valid Role", s)),
        }
    }
}

impl DeltaFunction {
    pub fn merge(&mut self, other: &Self) {
        if self.name.is_none() && other.name.is_none() == false {
            self.name = other.name.clone();
        }

        if let Some(arguments) = &other.arguments {
            if let Some(self_arguments) = &mut self.arguments {
                self_arguments.push_str(arguments);
            } else {
                self.arguments = Some(arguments.clone());
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.arguments.is_none()
    }
}
