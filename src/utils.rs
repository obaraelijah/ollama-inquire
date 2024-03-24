use std::{
    ffi::OsString,
    io::{Error, Write},
};

pub fn greet_user() -> Result<(), Error> {
    writeln!(std::io::stdout(), "Welcome to ask Ollama!")?;
    Ok(())
}

pub fn handle_help_or_version_request(program_args: &mut Vec<OsString>) -> Result<bool, Error> {
    let mut handled = false;
    if let Some(arg) = program_args.pop() {
        // Use pop to handle last argument
        match arg.to_str().unwrap_or_default() {
            "--help" => {
                print_help();
                handled = true;
            }
            "--version" => {
                println!("inquire-ollama version: {}", env!("CARGO_PKG_VERSION"));
                handled = true;
            }
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid argument",
                ))
            }
        }
    }
    Ok(handled)
}

pub fn parse_arguments(args: &mut Vec<OsString>) -> Result<(String, String), Error> {
    let mut model = String::from("mistrial");
    let mut question = String::new();

    for arg in args.iter() {
        let arg_str = arg.to_str().unwrap_or_default(); // Handle potential non-utf8 arguments

        if arg_str.starts_with("--model=") {
            let model_parts: Vec<&str> = arg_str.splitn(2, '=').collect();
            if model_parts.len() == 2 {
                model = model_parts[1].to_string();
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid --model argument",
                ));
            }
        } else {
            question.push_str(arg_str);
            question.push(' ');
        }
    }

    question = question.trim().to_string();
    Ok((model, question))
}

pub fn print_help() {
    println!("Usage: ask [OPTIONS] [PROMPT]");
    println!("Ask questions to Ollama.");
    println!("\nOptions:");
    println!("  --model=[MODEL]    Specify the model to use. Default is 'mistral' if no model is provided");
    println!("  --version          Show version information");
    println!("  [PROMPT]           The question to ask Ollama");
}
