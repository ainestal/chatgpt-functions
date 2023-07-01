//! A library to interact with OpenAI API to use GPT-3.5 and GPT-4 for chatbots.
//!
//! It takes care of the context and the state of the conversation, and provides a way to define
//! functions that can be called from the chatbot.
//!
//! # Usage
//! The main module to use for the chatbot is `chat_gpt`.
//! It provides a `ChatGPT` struct that can be used to interact with the GPT API.
//!
//! ## Example
//! ```no_run
//! use anyhow::Result;
//! use dotenv::dotenv;
//! use chatgpt_functions::chat_gpt::ChatGPTBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     dotenv().ok();
//!     let key = std::env::var("OPENAI_API_KEY")?;
//!     let mut gpt = ChatGPTBuilder::new().openai_api_token(key).build()?;
//!     let answer = gpt.completion_managed("Hello, how are you?".to_string()).await?;
//!     println!("{}", answer);
//!     Ok(())
//! }

// The main module to use, most of the use cases will only need this
pub mod chat_gpt;
// Internals, to be used by the library or in case more control is needed
pub mod chat_context;
pub mod chat_response;
pub mod function_specification;
pub mod message;

// Escape a string to be used in JSON
pub mod escape_json;
