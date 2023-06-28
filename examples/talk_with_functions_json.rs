use anyhow::{Context, Result};
use chatgpt_functions::{chat_gpt::ChatGPT, function_specification::FunctionSpecification};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let mut gpt = ChatGPT::new(key, None, None, None)?;

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
        // println!("Request: {}", chat_context);
        let answer = gpt.completion_managed(input).await?;
        // println!("Full answer: {}", answer.to_string());
        print_answer(&answer);
        println!("--------------------------------------");
    }
}

fn print_answer(answer: &chatgpt_functions::chat_response::ChatResponse) {
    for choice in &answer.choices {
        match choice.message.content {
            Some(ref content) => {
                println!("Answer: {}", content);
            }
            None => (),
        };
        match choice.message.name {
            Some(ref name) => {
                println!("Name: {}", name);
            }
            None => (),
        };
        match choice.message.function_call {
            Some(ref function_call) => {
                println!("Function call: {}", function_call);
            }
            None => (),
        };
    }
}
