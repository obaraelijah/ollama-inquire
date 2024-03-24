use spinners_rs::{Spinner, Spinners};
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn ollama_installed() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("command -v ollama")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn install_ollama() -> Result<Output, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg("curl https://ollama.ai/install.sh | sh")
        .output()
}

pub fn run_ollama(model: &str, question: &str) -> Result<Output, std::io::Error> {
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
