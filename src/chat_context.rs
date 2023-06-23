use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \"role\": {}, \"content\": {} }}",
            self.role, self.content
        )
    }
}

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

impl fmt::Display for ChatContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"model\": {}, \"messages\": [", self.model)?;
        for message in self.messages.iter() {
            write!(f, "{}, ", message)?;
        }
        write!(f, "] }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_message() {
        let message = Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        assert_eq!(
            message.to_string(),
            "{ \"role\": user, \"content\": Hello }"
        );
    }

    #[test]
    fn test_display_chat_context() {
        let mut chat_context = ChatContext::new("weather".to_string());
        chat_context.push(Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        });
        chat_context.push(Message {
            role: "bot".to_string(),
            content: "Hi".to_string(),
        });
        assert_eq!(
            chat_context.to_string(),
            "{ \"model\": weather, \"messages\": [{ \"role\": user, \"content\": Hello }, { \"role\": bot, \"content\": Hi }, ] }"
        );
    }
}
