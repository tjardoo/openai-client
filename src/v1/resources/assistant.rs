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
    /// The system instructions that the assistant uses. The maximum length is 32768 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types 'code_interpreter', 'retrieval', or 'function'.
    pub tools: Vec<AssistantTools>,
    /// A list of file IDs attached to this assistant. There can be a maximum of 20 files attached to the assistant.
    /// Files are ordered by their creation date in ascending order.
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CreateAssistantParameters {
    /// ID of the model to use.
    pub model: String,
    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The system instructions that the assistant uses. The maximum length is 32768 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types 'code_interpreter', 'retrieval', or 'function'.
    pub tools: Option<Vec<AssistantTools>>,
    /// A list of file IDs attached to this assistant. There can be a maximum of 20 files attached to the assistant.
    /// Files are ordered by their creation date in ascending order.
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantCodeInterpreterTool {
    /// The type of tool being defined: 'code_interpreter'.
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantRetrievalTool {
    /// The type of tool being defined: 'retrieval'.
    pub r#type: String,
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
pub struct ListAssistantsResponse {
    /// The object type, which is always 'list'.
    pub object: String,
    /// The list of assistants.
    pub data: Vec<Assistant>,
    /// ID of the first object in the list.
    pub first_id: Option<String>,
    /// ID of the last object in the list.
    pub last_id: Option<String>,
    /// Indicates whether there are more assistants to retrieve.
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListAssistantsParameters {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    pub limit: Option<u32>,
    /// Sort order by the created_at timestamp of the objects. asc for ascending order and desc for descending order.
    pub order_by: Option<String>,
    /// A cursor for use in pagination. after is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include after=obj_foo in order to fetch the next page of the list.
    pub after: Option<String>,
    /// A cursor for use in pagination. before is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include before=obj_foo in order to fetch the previous page of the list.
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AssistantTools {
    CodeInterpreter(AssistantCodeInterpreterTool),
    Retrieval(AssistantRetrievalTool),
    Function(AssistantFunctionTool),
}
