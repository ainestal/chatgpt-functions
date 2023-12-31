use anyhow::Result;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::escape_json::EscapeJson;

/// Builder for Message
pub struct MessageBuilder {
    role: Option<String>,
    content: Option<String>,
    name: Option<String>,
    function_call: Option<FunctionCall>,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            role: None,
            content: None,
            name: None,
            function_call: None,
        }
    }

    pub fn role(mut self, role: String) -> MessageBuilder {
        self.role = Some(role);
        self
    }

    pub fn content(mut self, content: String) -> MessageBuilder {
        self.content = Some(content);
        self
    }

    pub fn name(mut self, name: String) -> MessageBuilder {
        self.name = Some(name);
        self
    }

    pub fn function_call(mut self, function_call: FunctionCall) -> MessageBuilder {
        self.function_call = Some(function_call);
        self
    }

    pub fn build(self) -> Result<Message> {
        let role = self.role.unwrap_or_else(|| "user".to_string());
        let content = self.content.map(|c| c.escape_json());
        let name = self.name;
        let function_call = self.function_call;

        Ok(Message {
            role,
            content,
            name,
            function_call,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub name: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl Message {
    pub fn new(role: String) -> Message {
        Message {
            role,
            content: None,
            name: None,
            function_call: None,
        }
    }

    pub fn new_user_message(content: String) -> Message {
        let content = content.escape_json();
        Message {
            role: "user".to_string(),
            content: Some(content),
            name: None,
            function_call: None,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_function_call(&mut self, function_call: FunctionCall) {
        self.function_call = Some(function_call);
    }
}

/// A message sent by the user or the bot
///
/// Print valid JSON for Message, no commas if last field
/// Arguments are escaped to avoid issues with quotes and newlines
/// They break the JSON format and the API doesn't handle them well
///
/// # Notes
/// The API asks for content to be present in the message, even when it's an assistant message with a function call
/// https://platform.openai.com/docs/api-reference/chat/create
///
/// # Examples
///
/// ```
/// use chatgpt_functions::message::{FunctionCall, Message};
///
/// let mut message = Message::new("role".to_string());
/// assert_eq!(message.to_string(), "{\"role\":\"role\",\"content\":\"\"}".to_string());
///
/// message.set_content("content".to_string());
/// assert_eq!(
///    message.to_string(),
///    "{\"role\":\"role\",\"content\":\"content\"}".to_string()
/// );
///
/// message.set_name("name".to_string());
/// assert_eq!(
///    message.to_string(),
///    "{\"role\":\"role\",\"content\":\"content\",\"name\":\"name\"}".to_string()
/// );
///
/// message.set_function_call(FunctionCall {
///    name: "name".to_string(),
///    arguments: "arguments".to_string(),
/// });
/// assert_eq!(
///    message.to_string(),
///    "{\"role\":\"role\",\"content\":\"content\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":\"arguments\"}}".to_string()
/// );
/// ```
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"role\":\"{}\"", self.role)?;
        if let Some(content) = &self.content {
            write!(f, ",\"content\":\"{}\"", content.escape_json())?;
        } else {
            write!(f, ",\"content\":\"\"")?;
        }
        if let Some(name) = &self.name {
            write!(f, ",\"name\":\"{}\"", name)?;
        }
        if let Some(function_call) = &self.function_call {
            write!(f, ",\"function_call\":{}", function_call)?;
        }
        write!(f, "}}")
    }
}

// Print valid JSON for FunctionCall, no commas if last field
// Arguments are escaped to avoid issues with quotes and newlines
// They break the JSON format and the API doesn't handle them well
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"name\":\"{}\",\"arguments\":\"{}\"}}",
            self.name,
            self.arguments.escape_json()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_message() {
        let mut message = Message::new("role".to_string());
        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"\"}".to_string()
        );

        message.set_content(
            "content with \"quotes\" and a \nnewline, and other stuff like \\ \"\n\r\t\x08\x0C\""
                .to_string(),
        );
        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"content with \\\"quotes\\\" and a \\nnewline, and other stuff like \\\\ \\\"\\n\\r\\t\\b\\f\\\"\"}"
                .to_string()
        );

        message.set_name("name".to_string());
        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"content with \\\"quotes\\\" and a \\nnewline, and other stuff like \\\\ \\\"\\n\\r\\t\\b\\f\\\"\",\"name\":\"name\"}"
                .to_string()
        );

        let function_call = FunctionCall {
            name: "name".to_string(),
            arguments: "{\"example\":\"this\"}".to_string(),
        };
        message.set_function_call(function_call);
        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"content with \\\"quotes\\\" and a \\nnewline, and other stuff like \\\\ \\\"\\n\\r\\t\\b\\f\\\"\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}}".to_string()
        );
    }

    #[test]
    fn test_display_function_call_no_name() {
        let function_call = FunctionCall {
            name: "".to_string(),
            arguments: "{\"example\":\"this\"}".to_string(),
        };
        assert_eq!(
            function_call.to_string(),
            "{\"name\":\"\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}".to_string()
        );
    }

    #[test]
    fn test_display_function_call_no_arguments() {
        let function_call = FunctionCall {
            name: "name".to_string(),
            arguments: "{}".to_string(),
        };
        assert_eq!(
            function_call.to_string(),
            "{\"name\":\"name\",\"arguments\":\"{}\"}".to_string()
        );
    }

    #[test]
    fn test_display_function_call() {
        let function_call = FunctionCall {
            name: "name".to_string(),
            arguments: "{\"example\":\"this\"}".to_string(),
        };
        assert_eq!(
            function_call.to_string(),
            "{\"name\":\"name\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}".to_string()
        );
    }

    #[test]
    fn test_display_message_parsed_from_json_remove_newline() {
        let message = r#"{
            "role": "assistant",
            "content": null,
            "function_call": {
                "name": "completion_managed",
                "arguments": "{\n  \"content\": \"Hi model, how are you today?\"\n}"
            }
        }"#
        .to_string();
        let message_parsed: Message =
            serde_json::from_str(&message).expect("JSON was not well-formatted");

        // When we parse the JSON, we remove the newlines
        assert_eq!(message_parsed.role, "assistant".to_string());
        assert_eq!(message_parsed.content, None);

        // The API asks for content to be present in the message, even when it's an assistant message with a function call
        // https://platform.openai.com/docs/api-reference/chat/create
        assert_eq!(
            message_parsed.to_string(),
            "{\"role\":\"assistant\",\"content\":\"\",\"function_call\":{\"name\":\"completion_managed\",\"arguments\":\"{\\n  \\\"content\\\": \\\"Hi model, how are you today?\\\"\\n}\"}}".to_string()
        );

        // When we don't use our custom Display trait, the newlines are kept
        assert_eq!(
            message_parsed.function_call,
            Some(FunctionCall {
                name: "completion_managed".to_string(),
                arguments: "{\n  \"content\": \"Hi model, how are you today?\"\n}".to_string(),
            })
        );
    }

    #[test]
    fn test_message_new_user_message() {
        let message =
            Message::new_user_message("content with \"quotes\" and other' stuff \\".to_string());
        assert_eq!(
            message.to_string(),
            "{\"role\":\"user\",\"content\":\"content with \\\\\\\"quotes\\\\\\\" and other' stuff \\\\\\\\\"}".to_string()
        );
    }

    #[test]
    fn test_message_builder() {
        let message = MessageBuilder::new()
            .content("content with \"quotes\" and other/' stuff \\".to_string())
            .name("name".to_string())
            .role("role".to_string())
            .function_call(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            })
            .build()
            .expect("MessageBuilder failed");

        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"content with \\\\\\\"quotes\\\\\\\" and other/' stuff \\\\\\\\\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}}".to_string()
        );
    }
}
