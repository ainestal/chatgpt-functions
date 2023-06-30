use anyhow::{Context, Result};
use dotenv::dotenv;

use chatgpt_functions::chat_gpt::ChatGPTBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let mut gpt = ChatGPTBuilder::new().openai_api_token(key).build()?;

    println!("Initialised chatbot. Enter your message to start a conversation.");
    println!("Using:");
    println!("- Model: {}", gpt.chat_context.model);
    println!("- Session ID: {}", gpt.session_id);
    println!("You can quit by pressing Ctrl+C (linux), or Cmd+C (Mac).");
    println!("--------------------------------------");
    loop {
        println!("- Enter your message and press Enter:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read your input")?;
        input.pop(); // Remove the trailing newline

        println!("- AI:");
        // println!("Request: {}", gpt.chat_context);
        let answer = gpt.completion_managed(input).await?;
        // println!("Full answer: {}", answer.to_string());
        println!("{}", answer.content().expect("Failed to get the content"));
        println!("--------------------------------------");
    }
}
