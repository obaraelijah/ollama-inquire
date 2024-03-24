use spinners_rs::{Spinner, Spinners};
use std::env;
use std::thread;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::{Write, Error};
use std::ffi::OsString;

fn main() -> Result<(), Error> {
    greet_user()?;
    let mut program_args = env::args_os().skip(1).collect::<Vec<OsString>>(); 

    if program_args.is_empty() && !handle_help_or_version_request(&mut program_args)? {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "No question provided"));
    }
    
    let (model, question) = parse_arguments(&mut program_args)?;

    if !ollama_installed() {
        println!("Ollama is not installed. Installing Ollama...");
        install_ollama().expect("Failed to install Ollama");
    }

    run_ollama(&model, &question).expect("Failed to run Ollama");
    Ok(())
}

fn greet_user() -> Result<(), Error> {
    writeln!(std::io::stdout(), "Welcome to ask Ollama!")?;
    Ok(())
}

fn handle_help_or_version_request(program_args: &mut Vec<OsString>) -> Result<bool, Error> {
    let mut handled = false;
    if let Some(arg) = program_args.pop() { // Use pop to handle last argument
        match arg.to_str().unwrap_or_default() {
            "--help" => {
                print_help();
                handled = true;
            }
            "--version" => {
                println!("inquire-ollama version: {}", env!("CARGO_PKG_VERSION"));
                handled = true;
            }
            _ => return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid argument")),
        }
    }
    Ok(handled)
}

fn parse_arguments(args: &mut Vec<OsString>) -> Result<(String, String), Error> {
    let mut model = String::from("mistrial");
    let mut question = String::new();

    for arg in args.iter() {
        let arg_str = arg.to_str().unwrap_or_default(); // Handle potential non-utf8 arguments

        if arg_str.starts_with("--model=") {
            let model_parts: Vec<&str> = arg_str.splitn(2, '=').collect();
            if model_parts.len() == 2 {
                model = model_parts[1].to_string();
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid --model argument"));
            }
        } else {
            question.push_str(arg_str);
            question.push(' ');
        }
    }

    question = question.trim().to_string();
    Ok((model, question))
}



fn print_help() {
    println!("Usage: ask [OPTIONS] [PROMPT]");
    println!("Ask questions to Ollama.");
    println!("\nOptions:");
    println!("  --model=[MODEL]    Specify the model to use. Default is 'mistral' if no model is provided");
    println!("  --version          Show version information");
    println!("  [PROMPT]           The question to ask Ollama");
}

fn ollama_installed() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("command -v ollama")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_ollama() -> Result<Output, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg("curl https://ollama.ai/install.sh | sh")
        .output()
}


fn run_ollama(model: &str, question: &str) -> Result<Output, std::io::Error> {
    let loading_msg: String = format!("Running {}...", model);
    let mut spinner = Spinner::new(Spinners::Aesthetic, loading_msg);
    spinner.start();

    let output = Arc::new(Mutex::new(None));
    let output_clone = Arc::clone(&output);

    let model_clone = model.to_string();
    let question_clone = question.to_string();
    thread::spawn(move || {
        let command_output = Command::new("ollama")
            .arg("run")
            .arg(&model_clone)
            .arg(&question_clone)
            .stdout(Stdio::piped()) // Capture stdout
            .stderr(Stdio::piped()) // Capture stderr
            .output();

        let mut output = output_clone.lock().unwrap();
        *output = Some(command_output);
    });

    let mut command_output: Option<Output> = None;
    while command_output.is_none() {
        let output = output.lock().unwrap();
        if let Some(result) = &*output {
            command_output = result.as_ref().ok().cloned();
            break;
        }
        drop(output); // Release the lock before sleeping
        thread::sleep(Duration::from_millis(100));
    }

    spinner.stop();

    let output = command_output.unwrap();
    // Print the captured output
    println!("\r{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(output)
} 