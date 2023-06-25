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
