use anyhow::{Context, Result};

use crate::{chat_context::ChatContext, chat_response::ChatResponse};

const URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct ChatGPT {
    client: reqwest::Client,
    openai_api_token: String,
    pub session_id: String,
}

impl ChatGPT {
    pub fn new(openai_api_token: String, session_id: String) -> Result<ChatGPT> {
        let client = reqwest::Client::new();
        Ok(ChatGPT {
            client,
            openai_api_token,
            session_id,
        })
    }

    pub async fn completion(&mut self, chat_context: &ChatContext) -> Result<ChatResponse> {
        let response = self
            .client
            .post(URL)
            .bearer_auth(&self.openai_api_token)
            .header("Content-Type", "application/json")
            .json(&chat_context)
            .send()
            .await
            .context(format!("Failed to receive the response from {}", URL))?
            .text()
            .await
            .context("Failed to retrieve the content of the response")?;

        let answer: ChatResponse = serde_json::from_str(&response)?;
        Ok(answer)
    }
}
