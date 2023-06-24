use std::collections::HashMap;

use anyhow::{Context, Result};
use chatgpt_functions::function_specification::{FunctionSpecification, Parameters, Property};
use dotenv::dotenv;
use uuid::Uuid;

use chatgpt_functions::chat_context::ChatContext;
use chatgpt_functions::chat_gpt::ChatGPT;
use chatgpt_functions::message::Message;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let model = "gpt-3.5-turbo-0613".to_string();
    let key = std::env::var("OPENAI_API_KEY")?;
    let session_id = Uuid::new_v4().to_string();

    let mut gpt = ChatGPT::new(key, session_id.clone())?;
    let mut chat_context = ChatContext::new(model.clone());
    let mut properties = HashMap::new();
    properties.insert(
        "location".to_string(),
        Property {
            type_: "string".to_string(),
            description: Some("The city and state, e.g. San Francisco, CA".to_string()),
            enum_: None,
        },
    );

    let functions = FunctionSpecification {
        name: "get_current_weather".to_string(),
        description: "Get the current weather in a given location".to_string(),
        parameters: Parameters {
            type_: "object".to_string(),
            properties: properties,
            required: vec!["location".to_string()],
        },
    };

    chat_context.set_functions(functions);

    println!(
        "Initialised {} chatbot. Enter your message to start a conversation.",
        chat_context.model
    );
    println!("Using:");
    println!("- Model: {}", chat_context.model);
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

        chat_context.push_message(Message {
            role: "user".to_string(),
            content: Some(input),
            name: None,
            function_call: None,
        });

        println!("- AI:");
        println!("Request: {}", chat_context);
        let answer = gpt
            .completion(&chat_context)
            .await
            .context("Could not get an answer from GPT")?;
        println!("Full answer: {}", answer.to_string());
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
