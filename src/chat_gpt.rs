use anyhow::{Context, Result};
use uuid::Uuid;

use crate::{chat_context::ChatContext, chat_response::ChatResponse, message::Message};

const DEFAULT_MODEL: &str = "gpt-3.5-turbo-0613";
const URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct ChatGPT {
    client: reqwest::Client,
    openai_api_token: String,
    pub session_id: String,
    pub chat_context: ChatContext,
}

impl ChatGPT {
    pub fn new(
        openai_api_token: String,
        chat_context: Option<ChatContext>,
        session_id: Option<String>,
    ) -> Result<ChatGPT> {
        let client = reqwest::Client::new();
        let session_id = if let Some(session_id) = session_id {
            session_id
        } else {
            Uuid::new_v4().to_string()
        };
        let chat_context = if let Some(chat_context) = chat_context {
            chat_context
        } else {
            ChatContext::new(DEFAULT_MODEL.to_string())
        };
        Ok(ChatGPT {
            client,
            openai_api_token,
            session_id,
            chat_context,
        })
    }

    pub async fn completion(&mut self) -> Result<ChatResponse> {
        let response = self
            .client
            .post(URL)
            .bearer_auth(&self.openai_api_token)
            .header("Content-Type", "application/json")
            // Use Display trait to avoid sending None fields that the API would reject
            .body(self.chat_context.to_string())
            .send()
            .await
            .context(format!("Failed to receive the response from {}", URL))?
            .text()
            .await
            .context("Failed to retrieve the content of the response")?;

        let answer: ChatResponse = serde_json::from_str(&response)?;
        Ok(answer)
    }

    pub async fn completion_with_message(&mut self, message: Message) -> Result<ChatResponse> {
        self.push_message(message);
        self.completion().await
    }

    pub async fn completion_with_user_content(&mut self, content: String) -> Result<ChatResponse> {
        let message = Message::new_user_message(content);
        self.completion_with_message(message).await
    }

    /// This function is used to update the context with the response from the AI
    /// It assumes that there will only be one choice in the response
    /// It returns the response from the AI
    /// It does update the context with the response from the AI
    ///
    /// # Example
    /// ```
    /// use chatgpt_functions::chat_context::ChatContext;
    /// use chatgpt_functions::chat_gpt::ChatGPT;
    /// use chatgpt_functions::message::Message;
    /// use chatgpt_functions::chat_response::ChatResponse;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut gpt = ChatGPT::new("key".to_string(), None, None)?;
    ///     let mut message = Message::new("role".to_string());
    ///     message.set_content("Hi!".to_string());
    ///
    ///     let response_message = gpt.completion_with_message_updating_context(message).await?;
    ///
    ///     // The answer from the AI will be stored in the context
    ///     assert_eq!(response_message.content.unwrap(), "Hi, how are you?".to_string());
    ///     assert_eq!(gpt.chat_context.messages.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn completion_with_message_updating_context(
        &mut self,
        message: Message,
    ) -> Result<Message> {
        self.push_message(message);
        let response = self.completion().await?;
        self.push_message(response.choices[0].message.clone());
        Ok(response.choices[0].message.clone())
    }

    pub fn push_message(&mut self, message: Message) {
        self.chat_context.push_message(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_gpt_new() {
        let chat_gpt = ChatGPT::new("key".to_string(), None, None).unwrap();
        assert_eq!(chat_gpt.session_id.len(), 36);
        assert_eq!(chat_gpt.chat_context.model, DEFAULT_MODEL);
    }

    #[test]
    fn test_chat_gpt_new_with_session_id() {
        let session_id = "session_id".to_string();
        let chat_gpt = ChatGPT::new("key".to_string(), None, Some(session_id.clone())).unwrap();
        assert_eq!(chat_gpt.session_id, session_id);
    }

    #[test]
    fn test_chat_gpt_new_with_chat_context() {
        let chat_context = ChatContext::new("model".to_string());
        let chat_gpt = ChatGPT::new("key".to_string(), Some(chat_context), None).unwrap();
        assert_eq!(chat_gpt.chat_context.model, "model");
    }

    #[test]
    fn test_chat_gpt_new_with_session_id_and_chat_context() {
        let session_id = "session_id".to_string();
        let chat_context = ChatContext::new("model".to_string());
        let chat_gpt = ChatGPT::new(
            "key".to_string(),
            Some(chat_context.clone()),
            Some(session_id.clone()),
        )
        .unwrap();
        assert_eq!(chat_gpt.session_id, session_id);
        assert_eq!(chat_gpt.chat_context.model, "model");
    }
}
