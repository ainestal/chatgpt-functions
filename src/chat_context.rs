use std::fmt;

use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatContext {
    pub model: String,
    pub messages: Vec<Message>,
}

impl ChatContext {
    pub fn new(model: String) -> ChatContext {
        ChatContext {
            model,
            messages: Vec::new(),
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }
}

// Print valid JSON for ChatContext, no commas if last field
impl fmt::Display for ChatContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"model\":{},\"messages\":[", self.model)?;
        for (i, message) in self.messages.iter().enumerate() {
            write!(
                f,
                "{}{}",
                message,
                if i == self.messages.len() - 1 {
                    ""
                } else {
                    ","
                }
            )?;
        }
        write!(f, "]}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_chat_context() {
        let mut chat_context = ChatContext::new("weather".to_string());
        chat_context.push(Message {
            role: "user".to_string(),
            content: Some("Hello".to_string()),
            name: None,
            function_call: None,
        });
        chat_context.push(Message {
            role: "bot".to_string(),
            content: Some("Hi".to_string()),
            name: None,
            function_call: None,
        });
        assert_eq!(
            chat_context.to_string(),
            "{\"model\":weather,\"messages\":[{\"role\":user,\"content\":Hello},{\"role\":bot,\"content\":Hi}]}"
        );
    }
}
