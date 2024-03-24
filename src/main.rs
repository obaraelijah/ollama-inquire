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
    let model_specified = false;

    

}

fn print_help() {
    unimplemented!()
}