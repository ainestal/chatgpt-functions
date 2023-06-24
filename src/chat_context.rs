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
    pub fn new(model: String) -> ChatContext {
        ChatContext {
            model,
            messages: Vec::new(),
            functions: Vec::new(),
            function_call: None,
        }
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn set_functions(&mut self, functions: FunctionSpecification) {
        self.functions.push(functions);
    }

    pub fn set_function_call(&mut self, function_call: String) {
        self.function_call = Some(function_call);
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
        message::Message,
    };

    #[test]
    fn test_display_for_chat_context() {
        let mut chat_context = ChatContext::new("test_model".to_string());
        chat_context.push_message(Message {
            role: "role".to_string(),
            content: Some("Hello".to_string()),
            name: None,
            function_call: None,
        });
        chat_context.push_message(Message {
            role: "bot".to_string(),
            content: Some("Hi".to_string()),
            name: None,
            function_call: None,
        });
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
        let functions = FunctionSpecification {
            name: "test_function".to_string(),
            description: "a dummy function to test the chat context".to_string(),
            parameters: Parameters {
                type_: "object".to_string(),
                properties,
                required: vec!["location".to_string()],
            },
        };
        chat_context.set_functions(functions);

        // Add a message to the chat context
        chat_context.push_message(Message {
            role: "test".to_string(),
            content: Some("hi".to_string()),
            name: Some("test_function".to_string()),
            function_call: None, // Lets assume a function has not been called yet
        });

        // Print the chat context, with the model, the messages, the functions, and the function_call
        assert_eq!(
            chat_context.to_string(),
            "{\"model\":\"test_model\",\"messages\":[{\"role\":\"test\",\"content\":\"hi\",\"name\":\"test_function\"}],\"functions\":[{\"name\":\"test_function\",\"description\":\"a dummy function to test the chat context\",\"parameters\":{\"type\":\"object\",\"properties\":{\"location\":{\"type\":\"string\",\"description\":\"a dummy string\"}},\"required\":[\"location\"]}}]}"
        );
    }
}
