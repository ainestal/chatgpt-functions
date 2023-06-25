# chatgpt-functions

Library to interact with gpt in chat mode, using functions.

## Disclaimer

This is a work in progress. The API is not stable and will change.

# Requirements

- Rust 1.26.0 or higher
- OpenAI API key

# Usage

## Example without functions

```rust
use anyhow::{Context, Result};
use dotenv::dotenv;

use chatgpt_functions::chat_gpt::ChatGPT;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let mut gpt = ChatGPT::new(key, None, None)?;

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
        // println!("Request: {}", chat_context);
        let answer = gpt.completion_with_user_content(input).await?;
        // println!("Full answer: {}", answer.to_string());
        println!("{}", answer.choices[0].message);
        println!("--------------------------------------");
    }
}
```

## Example with functions

```rust
use std::collections::HashMap;

use anyhow::{Context, Result};
use chatgpt_functions::{
    chat_gpt::ChatGPT,
    function_specification::{FunctionSpecification, Parameters, Property},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let mut gpt = ChatGPT::new(key, None, None)?;

    let mut properties = HashMap::new();
    properties.insert(
        "location".to_string(),
        Property {
            type_: "string".to_string(),
            description: Some("The city and state, e.g. San Francisco, CA".to_string()),
            enum_: None,
        },
    );
    let function = FunctionSpecification {
        name: "get_current_weather".to_string(),
        description: Some("Get the current weather in a given location".to_string()),
        parameters: Some(Parameters {
            type_: "object".to_string(),
            properties: properties,
            required: vec!["location".to_string()],
        }),
    };

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
        // println!("Request: {}", chat_context);
        let answer = gpt.completion_with_user_content(input).await?;
        println!("Full answer: {}", answer.to_string());
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
