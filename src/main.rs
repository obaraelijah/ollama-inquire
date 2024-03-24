mod ollama;
mod utils;

use crate::ollama::{install_ollama, ollama_installed, run_ollama};
use crate::utils::{greet_user, handle_help_or_version_request, parse_arguments};

use std::env;
use std::ffi::OsString;
use std::io::Error;

fn main() -> Result<(), Error> {
    greet_user()?;
    let mut program_args = env::args_os().skip(1).collect::<Vec<OsString>>();

    if program_args.is_empty() && !handle_help_or_version_request(&mut program_args)? {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No question provided",
        ));
    }

    let (model, question) = parse_arguments(&mut program_args)?;

    if !ollama_installed() {
        println!("Ollama is not installed. Installing Ollama...");
        install_ollama().expect("Failed to install Ollama");
    }

    run_ollama(&model, &question).expect("Failed to run Ollama");
    Ok(())
}
