use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::chat_context::{ChatContext, Message};

const URL: &str = "https://api.openai.com/v1/chat/completions";

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

    pub async fn completion(&mut self, chat_context: &ChatContext) -> Result<Message> {
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
        Ok(answer.choices[0].message.clone())
    }
}
