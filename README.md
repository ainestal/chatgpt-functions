# chatgpt-functions

This crate provides a wrapper around the OpenAI API to use GPT-3.5 and GPT-4 for chatbots. It also provides a way to define functions that can be called from the chatbot.

## Disclaimer

This is a work in progress. The API is not stable and will change.

# Requirements

- Rust 1.70.0 or higher
- OpenAI API key. To get one:
  - Go to https://openai.com/
  - Sign up
  - Go to https://platform.openai.com/account/api-keys
  - Create a new API key
  - Copy the key and store it in a safe place

# Usage

Add the following to your `Cargo.toml`:

```rust
let mut gpt = ChatGPTBuilder::new().openai_api_token(key).build()?;
let input = "Prompt for the bot".to_string();
let answer = gpt.completion_managed(input).await?;
```

# Documentation

The documentation is available at https://docs.rs/chatgpt-functions

# Features

- [x] Chat with GPT-3.5 and GPT-4
- [x] Define functions that can be called from the chatbot

# Examples

You can find examples in the `examples` folder.

## Example without functions

```rust
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
        let answer = gpt.completion_managed(input).await?;
        println!("{}", answer.content().expect("Failed to get the content"));
        println!("--------------------------------------");
    }
}
```

## Example with functions

```rust
use anyhow::{Context, Result};
use chatgpt_functions::{
    chat_gpt::ChatGPTBuilder,
    function_specification::{FunctionSpecification},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let mut gpt = ChatGPTBuilder::new().openai_api_token(key).build()?;

    let json = r#"
        {
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            }
        }
        "#;
    let function: FunctionSpecification =
        serde_json::from_str(json).expect("Could not parse correctly the function specification");

    gpt.push_function(function);

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
        let answer = gpt.completion_managed(input).await?;
        println!("{}", answer.content().expect("Failed to get the content"));
        println!("--------------------------------------");
    }
}
```

## Example in bash of an interaction with GPT

```bash
curl https://api.openai.com/v1/chat/completions   -H "Content-Type: application/json"   -H "Authorization: Bearer $OPENAI_API_KEY"   -d '{
    "model": "gpt-3.5-turbo-0613",
    "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is the weather like in Madrid, Spain?"}],
    "functions": [{
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
                },
                "required": ["location"]
            }
        }],
    "function_call": "auto"
}'



{
  "id": "chatcmpl-7Ut7jsNlTUO9k9L5kBF0uDAyG19pK",
  "object": "chat.completion",
  "created": 1687596091,
  "model": "gpt-3.5-turbo-0613",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": null,
        "function_call": {
          "name": "get_current_weather",
          "arguments": "{\n  \"location\": \"Madrid, Spain\"\n}"
        }
      },
      "finish_reason": "function_call"
    }
  ],
  "usage": {
    "prompt_tokens": 90,
    "completion_tokens": 19,
    "total_tokens": 109
  }
}
```

# Contributing

Contributions are welcome! Please open an issue or a pull request.
