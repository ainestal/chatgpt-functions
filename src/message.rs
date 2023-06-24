use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub name: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

// Print valid JSON for Message, no commas if last field
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"role\":\"{}\"", self.role)?;
        if let Some(content) = &self.content {
            write!(f, ",\"content\":\"{}\"", content)?;
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
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"name\":\"{}\",\"arguments\":{}}}",
            self.name, self.arguments
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_message() {
        let message = Message {
            role: "role".to_string(),
            content: Some("content".to_string()),
            name: Some("name".to_string()),
            function_call: Some(FunctionCall {
                name: "name".to_string(),
                arguments: "{\"example\":\"this\"}".to_string(),
            }),
        };
        assert_eq!(
            message.to_string(),
            "{\"role\":\"role\",\"content\":\"content\",\"name\":\"name\",\"function_call\":{\"name\":\"name\",\"arguments\":{\"example\":\"this\"}}}".to_string()
        );
    }
}
