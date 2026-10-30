use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone, PartialEq)]
#[builder(name = "DecisionParametersBuilder")]
#[builder(setter(into, strip_option), default)]
pub struct DecisionParameters {
    /// The input to evaluate for every question.
    pub input: DecisionInput,
    /// The model to use. Only `gpt-6-luna` is supported.
    pub model: String,
    /// Ask 1-400 questions about the input. Answers come back in the same order.
    pub questions: Vec<DecisionQuestion>,
    /// Partition prompt cache reuse by caller-provided key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Options for prompt caching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<DecisionPromptCacheOptions>,
    /// Opaque caller-provided end-user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionInput {
    /// A text string.
    Text(String),
    /// An ordered array of messages, function calls and outputs, or custom tool calls and outputs.
    Items(Vec<DecisionInputItem>),
}

impl Default for DecisionInput {
    fn default() -> Self {
        DecisionInput::Text(String::new())
    }
}

impl From<&str> for DecisionInput {
    fn from(value: &str) -> Self {
        DecisionInput::Text(value.to_string())
    }
}

impl From<String> for DecisionInput {
    fn from(value: String) -> Self {
        DecisionInput::Text(value)
    }
}

impl From<Vec<DecisionInputItem>> for DecisionInput {
    fn from(value: Vec<DecisionInputItem>) -> Self {
        DecisionInput::Items(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DecisionInputItem {
    #[serde(rename = "message")]
    Message(DecisionInputMessage),
    #[serde(rename = "function_call")]
    FunctionCall(DecisionFunctionCall),
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(DecisionToolCallOutput),
    #[serde(rename = "custom_tool_call")]
    CustomToolCall(DecisionCustomToolCall),
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput(DecisionToolCallOutput),
}

impl DecisionInputItem {
    pub fn user(content: impl Into<DecisionContent>) -> Self {
        DecisionInputItem::Message(DecisionInputMessage::User {
            content: content.into(),
        })
    }

    pub fn developer(content: impl Into<DecisionTextContent>) -> Self {
        DecisionInputItem::Message(DecisionInputMessage::Developer {
            content: content.into(),
        })
    }

    pub fn system(content: impl Into<DecisionTextContent>) -> Self {
        DecisionInputItem::Message(DecisionInputMessage::System {
            content: content.into(),
        })
    }

    pub fn assistant(content: impl Into<DecisionAssistantContent>) -> Self {
        DecisionInputItem::Message(DecisionInputMessage::Assistant {
            content: content.into(),
            phase: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum DecisionInputMessage {
    User {
        content: DecisionContent,
    },
    Developer {
        content: DecisionTextContent,
    },
    System {
        content: DecisionTextContent,
    },
    Assistant {
        content: DecisionAssistantContent,
        /// The phase of an assistant message.
        #[serde(skip_serializing_if = "Option::is_none")]
        phase: Option<DecisionAssistantPhase>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionAssistantPhase {
    Commentary,
    FinalAnswer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionContent {
    Text(String),
    Parts(Vec<DecisionInputPart>),
}

impl From<&str> for DecisionContent {
    fn from(value: &str) -> Self {
        DecisionContent::Text(value.to_string())
    }
}

impl From<String> for DecisionContent {
    fn from(value: String) -> Self {
        DecisionContent::Text(value)
    }
}

impl From<Vec<DecisionInputPart>> for DecisionContent {
    fn from(value: Vec<DecisionInputPart>) -> Self {
        DecisionContent::Parts(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionAssistantContent {
    Text(String),
    Parts(Vec<DecisionAssistantPart>),
}

impl From<&str> for DecisionAssistantContent {
    fn from(value: &str) -> Self {
        DecisionAssistantContent::Text(value.to_string())
    }
}

impl From<String> for DecisionAssistantContent {
    fn from(value: String) -> Self {
        DecisionAssistantContent::Text(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionAssistantPart {
    OutputText { text: String },
    Refusal { refusal: String },
}

/// A content part of an input message, a tool output or a question's instructions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputPart {
    InputText {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<DecisionPromptCacheBreakpoint>,
    },
    InputImage {
        /// A base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported.
        image_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<DecisionImageDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<DecisionPromptCacheBreakpoint>,
    },
    /// A JSON object or an array of objects rendered as text.
    InputJson { data: serde_json::Value },
}

impl DecisionInputPart {
    pub fn text(text: impl Into<String>) -> Self {
        DecisionInputPart::InputText {
            text: text.into(),
            prompt_cache_breakpoint: None,
        }
    }

    pub fn image_url(image_url: impl Into<String>) -> Self {
        DecisionInputPart::InputImage {
            image_url: image_url.into(),
            detail: None,
            prompt_cache_breakpoint: None,
        }
    }

    pub fn json(data: serde_json::Value) -> Self {
        DecisionInputPart::InputJson { data }
    }
}

/// The content of a developer or system message: text or an array of text parts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionTextContent {
    Text(String),
    Parts(Vec<DecisionTextPart>),
}

impl From<&str> for DecisionTextContent {
    fn from(value: &str) -> Self {
        DecisionTextContent::Text(value.to_string())
    }
}

impl From<String> for DecisionTextContent {
    fn from(value: String) -> Self {
        DecisionTextContent::Text(value)
    }
}

impl From<Vec<DecisionTextPart>> for DecisionTextContent {
    fn from(value: Vec<DecisionTextPart>) -> Self {
        DecisionTextContent::Parts(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionTextPart {
    InputText {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<DecisionPromptCacheBreakpoint>,
    },
}

impl DecisionTextPart {
    pub fn text(text: impl Into<String>) -> Self {
        DecisionTextPart::InputText {
            text: text.into(),
            prompt_cache_breakpoint: None,
        }
    }
}

/// The output of a tool call: text or an array of text and image parts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionToolOutput {
    Text(String),
    Parts(Vec<DecisionToolOutputPart>),
}

impl From<&str> for DecisionToolOutput {
    fn from(value: &str) -> Self {
        DecisionToolOutput::Text(value.to_string())
    }
}

impl From<String> for DecisionToolOutput {
    fn from(value: String) -> Self {
        DecisionToolOutput::Text(value)
    }
}

impl From<Vec<DecisionToolOutputPart>> for DecisionToolOutput {
    fn from(value: Vec<DecisionToolOutputPart>) -> Self {
        DecisionToolOutput::Parts(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionToolOutputPart {
    InputText {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<DecisionPromptCacheBreakpoint>,
    },
    InputImage {
        /// A base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported.
        image_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<DecisionImageDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<DecisionPromptCacheBreakpoint>,
    },
}

impl DecisionToolOutputPart {
    pub fn text(text: impl Into<String>) -> Self {
        DecisionToolOutputPart::InputText {
            text: text.into(),
            prompt_cache_breakpoint: None,
        }
    }

    pub fn image_url(image_url: impl Into<String>) -> Self {
        DecisionToolOutputPart::InputImage {
            image_url: image_url.into(),
            detail: None,
            prompt_cache_breakpoint: None,
        }
    }
}

/// The instructions of a question: text or an array of text, image and JSON parts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionQuestionContent {
    Text(String),
    Parts(Vec<DecisionQuestionInputPart>),
}

impl From<&str> for DecisionQuestionContent {
    fn from(value: &str) -> Self {
        DecisionQuestionContent::Text(value.to_string())
    }
}

impl From<String> for DecisionQuestionContent {
    fn from(value: String) -> Self {
        DecisionQuestionContent::Text(value)
    }
}

impl From<Vec<DecisionQuestionInputPart>> for DecisionQuestionContent {
    fn from(value: Vec<DecisionQuestionInputPart>) -> Self {
        DecisionQuestionContent::Parts(value)
    }
}

/// A content part of a question's instructions. Unlike [`DecisionInputPart`], it has no prompt cache breakpoint.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionQuestionInputPart {
    InputText {
        text: String,
    },
    InputImage {
        /// A base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported.
        image_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<DecisionImageDetail>,
    },
    /// A JSON object or an array of objects rendered as text.
    InputJson {
        data: serde_json::Value,
    },
}

impl DecisionQuestionInputPart {
    pub fn text(text: impl Into<String>) -> Self {
        DecisionQuestionInputPart::InputText { text: text.into() }
    }

    pub fn image_url(image_url: impl Into<String>) -> Self {
        DecisionQuestionInputPart::InputImage {
            image_url: image_url.into(),
            detail: None,
        }
    }

    pub fn json(data: serde_json::Value) -> Self {
        DecisionQuestionInputPart::InputJson { data }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionImageDetail {
    Low,
    High,
    Auto,
    Original,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionPromptCacheBreakpoint {
    /// The breakpoint mode. Always `explicit`.
    pub mode: DecisionPromptCacheBreakpointMode,
}

impl Default for DecisionPromptCacheBreakpoint {
    fn default() -> Self {
        Self {
            mode: DecisionPromptCacheBreakpointMode::Explicit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionPromptCacheBreakpointMode {
    Explicit,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct DecisionPromptCacheOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DecisionPromptCacheMode>,
    /// The minimum lifetime of every cache breakpoint. Currently only `30m` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionPromptCacheMode {
    Implicit,
    Explicit,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionFunctionCall {
    pub arguments: String,
    pub call_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DecisionItemStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionCustomToolCall {
    pub call_id: String,
    pub input: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DecisionItemStatus>,
}

/// The output of a function call or custom tool call. The `call_id` must match a call in the input.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionToolCallOutput {
    pub call_id: String,
    /// Text evidence or an ordered list of text and image parts.
    pub output: DecisionToolOutput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DecisionItemStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionItemStatus {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionQuestion {
    /// Estimate how likely it is that a statement about the input is true.
    Predicate {
        instructions: DecisionQuestionContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// Choose from the supplied options based on the input.
    Choice {
        /// Between 2 and 255 unique choices.
        choices: Vec<DecisionChoice>,
        instructions: DecisionQuestionContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// Rate the input against the supplied ordered levels.
    Score {
        instructions: DecisionQuestionContent,
        /// Between 2 and 10 ordered levels.
        levels: Vec<DecisionScoreLevel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}

impl DecisionQuestion {
    pub fn predicate(name: &str, instructions: impl Into<DecisionQuestionContent>) -> Self {
        DecisionQuestion::Predicate {
            instructions: instructions.into(),
            name: Some(name.to_string()),
        }
    }

    pub fn choice(
        name: &str,
        instructions: impl Into<DecisionQuestionContent>,
        choices: Vec<DecisionChoice>,
    ) -> Self {
        DecisionQuestion::Choice {
            choices,
            instructions: instructions.into(),
            name: Some(name.to_string()),
        }
    }

    pub fn score(
        name: &str,
        instructions: impl Into<DecisionQuestionContent>,
        levels: Vec<DecisionScoreLevel>,
    ) -> Self {
        DecisionQuestion::Score {
            instructions: instructions.into(),
            levels,
            name: Some(name.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionChoice {
    pub value: DecisionChoiceValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DecisionChoice {
    pub fn new(value: impl Into<DecisionChoiceValue>, description: Option<&str>) -> Self {
        Self {
            value: value.into(),
            description: description.map(str::to_string),
        }
    }
}

/// Choice values are typed: a string and a boolean with the same text are distinct.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DecisionChoiceValue {
    Boolean(bool),
    Text(String),
}

impl From<&str> for DecisionChoiceValue {
    fn from(value: &str) -> Self {
        DecisionChoiceValue::Text(value.to_string())
    }
}

impl From<String> for DecisionChoiceValue {
    fn from(value: String) -> Self {
        DecisionChoiceValue::Text(value)
    }
}

impl From<bool> for DecisionChoiceValue {
    fn from(value: bool) -> Self {
        DecisionChoiceValue::Boolean(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionScoreLevel {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DecisionScoreLevel {
    pub fn new(label: &str, description: Option<&str>) -> Self {
        Self {
            label: label.to_string(),
            description: description.map(str::to_string),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionResponse {
    /// Answers in question order.
    pub answers: Vec<DecisionAnswer>,
    /// The model used.
    pub model: String,
    /// Token usage for the request.
    pub usage: DecisionUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionAnswer {
    Predicate {
        name: Option<String>,
        /// The probability that the statement is true.
        probability: f64,
    },
    Choice {
        choice: DecisionChoiceValue,
        confidence: f64,
        name: Option<String>,
        probabilities: Vec<DecisionChoiceProbability>,
    },
    Score {
        confidence: f64,
        name: Option<String>,
        probabilities: Vec<DecisionScoreProbability>,
        /// The expected score.
        score: f64,
    },
    /// The model declined to answer this question. Other questions can still receive answers.
    Refusal { name: Option<String> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionChoiceProbability {
    pub probability: f64,
    pub value: DecisionChoiceValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionScoreProbability {
    pub label: String,
    pub probability: f64,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionUsage {
    pub input_tokens: i64,
    pub input_tokens_details: DecisionInputTokensDetails,
    pub output_tokens: i64,
    pub output_tokens_details: DecisionOutputTokensDetails,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionInputTokensDetails {
    pub cache_write_tokens: i64,
    pub cached_tokens: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DecisionOutputTokensDetails {
    pub reasoning_tokens: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_decision_parameters_serialization() {
        let parameters = DecisionParametersBuilder::default()
            .model("gpt-6-luna")
            .input("The package arrived with a broken screen.")
            .questions(vec![
                DecisionQuestion::predicate("damaged", "Does the customer report a damaged item?"),
                DecisionQuestion::choice(
                    "tone",
                    "What is the tone?",
                    vec![
                        DecisionChoice::new("angry", None),
                        DecisionChoice::new(true, Some("flag")),
                    ],
                ),
                DecisionQuestion::score(
                    "urgency",
                    DecisionQuestionContent::Parts(vec![DecisionQuestionInputPart::text(
                        "How urgent?",
                    )]),
                    vec![
                        DecisionScoreLevel::new("low", None),
                        DecisionScoreLevel::new("high", Some("today")),
                    ],
                ),
            ])
            .build()
            .unwrap();

        assert_eq!(
            serde_json::to_value(&parameters).unwrap(),
            json!({
                "input": "The package arrived with a broken screen.",
                "model": "gpt-6-luna",
                "questions": [
                    {"type": "predicate", "instructions": "Does the customer report a damaged item?", "name": "damaged"},
                    {"type": "choice", "choices": [{"value": "angry"}, {"value": true, "description": "flag"}], "instructions": "What is the tone?", "name": "tone"},
                    {"type": "score", "instructions": [{"type": "input_text", "text": "How urgent?"}], "levels": [{"label": "low"}, {"label": "high", "description": "today"}], "name": "urgency"}
                ]
            })
        );
    }

    #[test]
    fn test_decision_input_items_serialization() {
        let input = DecisionInput::Items(vec![
            DecisionInputItem::system("Be strict."),
            DecisionInputItem::user("Hi"),
            DecisionInputItem::FunctionCall(DecisionFunctionCall {
                arguments: "{}".to_string(),
                call_id: "c1".to_string(),
                name: "lookup".to_string(),
                id: None,
                namespace: None,
                status: None,
            }),
            DecisionInputItem::FunctionCallOutput(DecisionToolCallOutput {
                call_id: "c1".to_string(),
                output: "ok".into(),
                id: None,
                status: Some(DecisionItemStatus::Completed),
            }),
        ]);

        assert_eq!(
            serde_json::to_value(&input).unwrap(),
            json!([
                {"type": "message", "role": "system", "content": "Be strict."},
                {"type": "message", "role": "user", "content": "Hi"},
                {"type": "function_call", "arguments": "{}", "call_id": "c1", "name": "lookup"},
                {"type": "function_call_output", "call_id": "c1", "output": "ok", "status": "completed"}
            ])
        );
    }

    #[test]
    fn test_decision_strict_content_serialization() {
        let developer = DecisionInputItem::developer(vec![DecisionTextPart::text("Be strict.")]);
        let output = DecisionToolCallOutput {
            call_id: "c1".to_string(),
            output: vec![
                DecisionToolOutputPart::text("see image"),
                DecisionToolOutputPart::image_url("https://example.com/a.png"),
            ]
            .into(),
            id: None,
            status: None,
        };
        let question = DecisionQuestion::predicate(
            "q",
            vec![
                DecisionQuestionInputPart::text("Check"),
                DecisionQuestionInputPart::json(json!({"a": 1})),
            ],
        );

        assert_eq!(
            serde_json::to_value(&developer).unwrap(),
            json!({"type": "message", "role": "developer", "content": [{"type": "input_text", "text": "Be strict."}]})
        );
        assert_eq!(
            serde_json::to_value(&output).unwrap(),
            json!({"call_id": "c1", "output": [{"type": "input_text", "text": "see image"}, {"type": "input_image", "image_url": "https://example.com/a.png"}]})
        );
        assert_eq!(
            serde_json::to_value(&question).unwrap(),
            json!({"type": "predicate", "instructions": [{"type": "input_text", "text": "Check"}, {"type": "input_json", "data": {"a": 1}}], "name": "q"})
        );
    }

    #[test]
    fn test_decision_response_deserialization() {
        let response: DecisionResponse = serde_json::from_value(json!({
            "model": "gpt-6-luna",
            "answers": [
                {"type": "predicate", "name": "damaged", "probability": 0.95},
                {"type": "choice", "name": null, "choice": "calm", "confidence": 0.8,
                 "probabilities": [{"value": "calm", "probability": 0.8}, {"value": true, "probability": 0.2}]},
                {"type": "score", "name": "urgency", "score": 1.7, "confidence": 0.55,
                 "probabilities": [{"label": "low", "probability": 0.3, "value": 0}]},
                {"type": "refusal", "name": null}
            ],
            "usage": {
                "input_tokens": 42,
                "input_tokens_details": {"cached_tokens": 0, "cache_write_tokens": 0},
                "output_tokens": 0,
                "output_tokens_details": {"reasoning_tokens": 0},
                "total_tokens": 42
            }
        }))
        .unwrap();

        assert_eq!(response.answers.len(), 4);
        assert_eq!(
            response.answers[0],
            DecisionAnswer::Predicate {
                name: Some("damaged".to_string()),
                probability: 0.95
            }
        );
        assert_eq!(response.answers[3], DecisionAnswer::Refusal { name: None });
        assert_eq!(response.usage.total_tokens, 42);
    }
}
