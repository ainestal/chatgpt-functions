use serde::{Deserialize, Serialize};
use std::fmt;

use crate::message::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    index: u64,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    pub choices: Vec<Choice>,
    usage: Usage,
}

impl ChatResponse {
    pub fn content(&self) -> Option<String> {
        match self.choices.first() {
            Some(choice) => {
                if let Some(c) = choice.message.content.clone() {
                    Some(c)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn function_call(&self) -> Option<(String, String)> {
        match self.choices.first() {
            Some(choice) => {
                if let Some(f) = choice.message.function_call.clone() {
                    Some((f.name, f.arguments))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Returns the message of the first choice
    /// This is the message that the bot will send
    pub fn message(&self) -> Option<Message> {
        match self.choices.first() {
            Some(choice) => Some(choice.message.clone()),
            None => None,
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"index\":{},\"message\":{},\"finish_reason\":\"{}\"}}",
            self.index, self.message, self.finish_reason
        )
    }
}

impl fmt::Display for ChatResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"id\":\"{}\",\"object\":\"{}\",\"created\":{},\"choices\":[",
            self.id, self.object, self.created
        )?;
        for (i, choice) in self.choices.iter().enumerate() {
            write!(
                f,
                "{}{}",
                choice,
                if i == self.choices.len() - 1 { "" } else { "," }
            )?;
        }
        write!(f, "],\"usage\":{}}}", self.usage)
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"prompt_tokens\":{},\"completion_tokens\":{},\"total_tokens\":{}}}",
            self.prompt_tokens, self.completion_tokens, self.total_tokens
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{FunctionCall, MessageBuilder};

    #[test]
    fn test_last_content() {
        let message = MessageBuilder::new()
            .content("content".to_string())
            .build()
            .unwrap();
        let chat_response = ChatResponse {
            id: "id".to_string(),
            object: "object".to_string(),
            created: 0,
            choices: vec![Choice {
                index: 0,
                message: message.clone(),
                finish_reason: "finish_reason".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        };
        assert_eq!(chat_response.content(), Some("content".to_string()));
    }

    #[test]
    fn test_last_function_call() {
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("content".to_string())
            .name("name".to_string())
            .function_call(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            })
            .build()
            .expect("Failed to build message");

        let chat_response = ChatResponse {
            id: "id".to_string(),
            object: "object".to_string(),
            created: 0,
            choices: vec![Choice {
                index: 0,
                message: message.clone(),
                finish_reason: "finish_reason".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        };
        assert_eq!(
            chat_response.function_call(),
            Some(("name".to_string(), "{\"example\":\"this\"}".to_string()))
        );
    }

    #[test]
    fn test_message() {
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("content".to_string())
            .name("name".to_string())
            .function_call(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            })
            .build()
            .expect("Failed to build message");

        let chat_response = ChatResponse {
            id: "id".to_string(),
            object: "object".to_string(),
            created: 0,
            choices: vec![Choice {
                index: 0,
                message: message.clone(),
                finish_reason: "finish_reason".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        };
        assert_eq!(chat_response.message(), Some(message),);
    }

    #[test]
    fn test_display_for_choice() {
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("content".to_string())
            .name("name".to_string())
            .function_call(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            })
            .build()
            .expect("Failed to build message");
        let choice = Choice {
            index: 0,
            message: message.clone(),
            finish_reason: "finish_reason".to_string(),
        };
        assert_eq!(
            format!("{}", choice),
            "{\"index\":0,\"message\":{\"role\":\"role\",\"content\":\"content\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}},\"finish_reason\":\"finish_reason\"}"
        );
    }

    #[test]
    fn test_display_chat_response() {
        let message = MessageBuilder::new()
            .role("role".to_string())
            .content("content".to_string())
            .name("name".to_string())
            .function_call(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            })
            .build()
            .expect("Failed to build message");
        let chat_response = ChatResponse {
            id: "id".to_string(),
            object: "object".to_string(),
            created: 0,
            choices: vec![Choice {
                index: 0,
                message: message.clone(),
                finish_reason: "finish_reason".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        };
        assert_eq!(
            format!("{}", chat_response),
            "{\"id\":\"id\",\"object\":\"object\",\"created\":0,\"choices\":[{\"index\":0,\"message\":{\"role\":\"role\",\"content\":\"content\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":\"{\\\"example\\\":\\\"this\\\"}\"}},\"finish_reason\":\"finish_reason\"}],\"usage\":{\"prompt_tokens\":0,\"completion_tokens\":0,\"total_tokens\":0}}"
        );
    }

    #[test]
    fn test_display_usage() {
        let usage = Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        };
        assert_eq!(
            format!("{}", usage),
            "{\"prompt_tokens\":0,\"completion_tokens\":0,\"total_tokens\":0}"
        );
    }
}
