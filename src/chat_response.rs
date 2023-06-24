use serde::{Deserialize, Serialize};
use std::fmt;

use crate::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    index: u64,
    message: Message,
    finish_reason: String,
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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"index\":{},\"message\":{},\"finish_reason\":{}}}",
            self.index, self.message, self.finish_reason
        )
    }
}

impl fmt::Display for ChatResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"id\":{},\"object\":{},\"created\":{},\"choices\":[",
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
    use crate::message::FunctionCall;

    #[test]
    fn test_display_for_choice() {
        let choice = Choice {
            index: 0,
            message: Message {
                role: "role".to_string(),
                content: Some("content".to_string()),
                name: Some("name".to_string()),
                function_call: Some(FunctionCall {
                    name: "name".to_string(),
                    arguments: "{\"example\":\"this\"}".to_string(),
                }),
            },
            finish_reason: "finish_reason".to_string(),
        };
        assert_eq!(
            format!("{}", choice),
            "{\"index\":0,\"message\":{\"role\":role,\"content\":content,\"name\":name,\"function_call\":{\"name\":name,\"arguments\":{\"example\":\"this\"}}},\"finish_reason\":finish_reason}"
        );
    }

    #[test]
    fn test_display_chat_response() {
        let chat_response = ChatResponse {
            id: "id".to_string(),
            object: "object".to_string(),
            created: 0,
            choices: vec![Choice {
                index: 0,
                message: Message {
                    role: "role".to_string(),
                    content: Some("content".to_string()),
                    name: Some("name".to_string()),
                    function_call: Some(FunctionCall {
                        name: "name".to_string(),
                        arguments: "{\"example\":\"this\"}".to_string(),
                    }),
                },
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
            "{\"id\":id,\"object\":object,\"created\":0,\"choices\":[{\"index\":0,\"message\":{\"role\":role,\"content\":content,\"name\":name,\"function_call\":{\"name\":name,\"arguments\":{\"example\":\"this\"}}},\"finish_reason\":finish_reason}],\"usage\":{\"prompt_tokens\":0,\"completion_tokens\":0,\"total_tokens\":0}}"
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