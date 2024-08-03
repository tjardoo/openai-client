use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Assistant {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always assistant.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub created_at: u32,
    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the model to use.
    pub model: String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types code_interpreter, file_search, or function.
    pub tools: Option<Vec<AssistantTools>>,
    /// A set of resources that are used by the assistant's tools.
    /// The resources are specific to the type of tool.
    /// For example, the code_interpreter tool requires a list of file IDs, while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<AssistantToolResource>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AssistantResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "AssistantParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct AssistantParameters {
    /// ID of the model to use.
    pub model: String,
    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types code_interpreter, file_search, or function.
    pub tools: Option<Vec<AssistantTools>>,
    /// A set of resources that are used by the assistant's tools.
    /// The resources are specific to the type of tool.
    /// For example, the code_interpreter tool requires a list of file IDs, while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<AssistantToolResource>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AssistantResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantToolResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    code_interpreter: Option<AssistantToolResourceCodeInterpreter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_search: Option<AssistantToolResourceFileSearch>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantToolResourceCodeInterpreter {
    /// A list of file IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool.
    file_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantToolResourceFileSearch {
    /// The ID of the vector store attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    vector_store_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantCodeInterpreterTool {
    /// The type of tool being defined: 'code_interpreter'.
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantFileSearchTool {
    /// The type of tool being defined: 'file_search'.
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<AssistantFileSearch>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantFunctionTool {
    /// The type of tool being defined: 'function'.
    pub r#type: String,
    pub function: AssistantFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantFunction {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the functions accepts, described as a JSON Schema object.
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantFileSearch {
    /// The maximum number of results the file search tool should output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssistantResponseFormat {
    Auto,
    #[serde(untagged)]
    JsonObject {
        /// Must be one of text or json_object.
        #[serde(rename = "type")]
        r#type: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantFile {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always 'assistant.file'.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the assistant file was created.
    pub created_at: u32,
    /// The assistant ID that the file is attached to.
    pub assistant_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "ToolOutputsParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct ToolOutputsParameters {
    /// A list of tools for which the outputs are being submitted.
    pub tool_outputs: Vec<ToolOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolOutput {
    /// The ID of the tool call in the 'required_action' object within the run object the output is being submitted for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// The output of the tool call to be submitted to continue the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AssistantTools {
    CodeInterpreter(AssistantCodeInterpreterTool),
    FileSearch(AssistantFileSearchTool),
    Function(AssistantFunctionTool),
}
