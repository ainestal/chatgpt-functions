use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{function_specification::FunctionSpecification, message::Message};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatContext {
    pub model: String,
    pub messages: Vec<Message>,
    pub functions: Vec<FunctionSpecification>,
    pub function_call: Option<String>,
}

impl ChatContext {
    /// Creates a new ChatContext with a model name
    /// as a string. This is an internal function used by other functions.
    pub fn new(model: String) -> ChatContext {
        ChatContext {
            model,
            messages: Vec::new(),
            functions: Vec::new(),
            function_call: None,
        }
    }

    /// Pushes a message in the chat context
    /// as a Message. This is an internal function used by other functions.
    /// It is recommended to use ChatGPT.push_message()
    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Sets the messages in the chat context
    /// as a vector of Message.
    /// This is an internal function used by other functions.
    pub fn set_messages(&mut self, messages: Vec<Message>) {
        self.messages = messages;
    }

    /// Pushes a function in the chat context
    /// as a FunctionSpecification.
    /// This is an internal function used by other functions.
    /// It is recommended to use ChatGPT.push_function()
    pub fn push_function(&mut self, functions: FunctionSpecification) {
        self.functions.push(functions);
    }

    /// Sets the functions in the chat context
    /// as a vector of FunctionSpecification.
    /// This is an internal function used by other functions.
    pub fn set_functions(&mut self, functions: Vec<FunctionSpecification>) {
        self.functions = functions;
    }

    /// Sets the last message sent by the user or the bot
    /// as a string. This is an internal function used by other functions.
    pub fn set_function_call(&mut self, function_call: String) {
        self.function_call = Some(function_call);
    }

    /// Returns the last message sent by the user or the bot
    /// as a string. This is an internal function used by other functions.
    /// It is recommended to use ChatGPT.last_content()
    pub fn last_content(&self) -> Option<String> {
        match self.messages.last() {
            Some(message) => {
                if let Some(c) = message.content.clone() {
                    Some(c)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Returns the last function call in the chat context
    /// as a tuple of the function name and the arguments.
    /// This is an internal function used by other functions.
    /// It is recommended to use ChatGPT.last_function_call()
    pub fn last_function_call(&self) -> Option<(String, String)> {
        match self.messages.last() {
            Some(message) => {
                if let Some(f) = message.function_call.clone() {
                    Some((f.name, f.arguments))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

// Print valid JSON for ChatContext, no commas if last field
impl fmt::Display for ChatContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"model\":\"{}\"", self.model)?;
        if !self.messages.is_empty() {
            write!(f, ",\"messages\":[")?;
            for (i, message) in self.messages.iter().enumerate() {
                write!(f, "{}", message)?;
                if i < self.messages.len() - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
        }
        if self.functions.len() > 0 {
            write!(f, ",\"functions\":[")?;
            for (i, function) in self.functions.iter().enumerate() {
                write!(f, "{}", function)?;
                if i < self.functions.len() - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
        }
        if let Some(function_call) = &self.function_call {
            write!(f, ",\"function_call\":\"{}\"", function_call)?;
        }
        write!(f, "}}")
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        function_specification::{Parameters, Property},
        message::MessageBuilder,
    };

    #[test]
    fn test_display_for_chat_context() {
        let mut chat_context = ChatContext::new("test_model".to_string());
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("Hello".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        let message = MessageBuilder::new()
            .role("bot".to_string())
            .content("Hi".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        assert_eq!(
            chat_context.to_string(),
            "{\"model\":\"test_model\",\"messages\":[{\"role\":\"role\",\"content\":\"Hello\"},{\"role\":\"bot\",\"content\":\"Hi\"}]}"
        );
    }

    #[test]
    fn test_display_chat_context_with_functions() {
        let mut chat_context = ChatContext::new("test_model".to_string());

        // Add a function to the chat context
        let mut properties = HashMap::new();
        properties.insert(
            "location".to_string(),
            Property {
                type_: "string".to_string(),
                description: Some("a dummy string".to_string()),
                enum_: None,
            },
        );
        let function = FunctionSpecification {
            name: "test_function".to_string(),
            description: Some("a dummy function to test the chat context".to_string()),
            parameters: Some(Parameters {
                type_: "object".to_string(),
                properties,
                required: vec!["location".to_string()],
            }),
        };
        chat_context.push_function(function);

        // Add a message to the chat context
        let message = MessageBuilder::new()
            .role("test".to_string())
            .content("hi".to_string())
            .name("test_function".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);

        // Print the chat context, with the model, the messages, the functions, and the function_call
        assert_eq!(
            chat_context.to_string(),
            "{\"model\":\"test_model\",\"messages\":[{\"role\":\"test\",\"content\":\"hi\",\"name\":\"test_function\"}],\"functions\":[{\"name\":\"test_function\",\"description\":\"a dummy function to test the chat context\",\"parameters\":{\"type\":\"object\",\"properties\":{\"location\":{\"type\":\"string\",\"description\":\"a dummy string\"}},\"required\":[\"location\"]}}]}"
        );
    }

    #[test]
    fn test_last_content() {
        let mut chat_context = ChatContext::new("model".to_string());

        // Test with no messages
        assert_eq!(chat_context.last_content(), None);

        // Test with a message with no content
        let message = MessageBuilder::new()
            .role("role".to_string())
            .name("name".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        assert_eq!(chat_context.last_content(), None);

        // Test with a message with content
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("content".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        assert_eq!(chat_context.last_content(), Some("content".to_string()));
    }

    #[test]
    fn test_last_function_call() {
        let mut chat_context = ChatContext::new("model".to_string());

        // Test with no messages
        assert_eq!(chat_context.last_content(), None);

        // Test with a message with no function call
        let message = MessageBuilder::new()
            .role("role".to_string())
            .name("name".to_string())
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        assert_eq!(chat_context.last_content(), None);

        // Test with a message with function call
        use crate::message::FunctionCall;
        let message = MessageBuilder::new()
            .role("role".to_string())
            .function_call(FunctionCall {
                name: "function".to_string(),
                arguments: "arguments".to_string(),
            })
            .build()
            .expect("Failed to build message");
        chat_context.push_message(message);
        assert_eq!(
            chat_context.last_function_call(),
            Some(("function".to_string(), "arguments".to_string()))
        );
    }
}
