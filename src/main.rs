use spinners_rs::{Spinner, Spinners};
use std::env;
use std::thread;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 1 && args[1] == "--help" {
        print_help();
        return;
    }

    if args.len() >= 1 && args[1] == "--version" {
        println!("ollama-inquire version: {}", env!("CARGO_PKG_VERSION"))
    }
    
    let mut model = String::from("mistrial");
    let mut question = String::from("Please enter your question: ");

    let mut args_iter = args.iter().skip(1);
    let mut model_specified = false;

    if let Some(first_arg) = args_iter.next() {
        if first_arg.starts_with("--model") {
            let model_parts: Vec<&str> = first_arg.splitn(2, '=').collect();
            if model_parts.len() == 2 {
                model = model_parts[1].to_string();
                model_specified = true;
            }
        } else {
            question.push_str(first_arg);
            question.push(' ');
        }
    }

    let mut new_question = String::new();

    for arg in args_iter {
        if !model_specified || arg != &model {
            new_question.push_str(arg);
            new_question.push(' ');
        }
    }

    if new_question.len() > 0 {
        question = new_question;
    }

    question = question.trim().to_string();

}

fn print_help() {
    println!("Usage: ask [OPTIONS] [PROMPT]");
    println!("Ask questions to Ollama.");
    println!("\nOptions:");
    println!("  --model=[MODEL]    Specify the model to use. Default is 'mistral' if no model is provided");
    println!("  --version          Show version information");
    println!("  [PROMPT]           The question to ask Ollama");
}
