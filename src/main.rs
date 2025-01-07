mod errors;

use std::env;
use std::io::Write;
use crate::errors::ErrorReporter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let bytes: String = std::fs::read_to_string(path).expect("Failed to read file");
    let result = run(&bytes);
    if result.had_error {
        std::process::exit(64);
    }
}

fn run_prompt(){
    let mut buffer = String::new();

    loop {
        buffer.clear();
        print!("> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().read_line(&mut buffer);
        if line.is_ok() {
            let buffer_result = buffer.trim();
            if buffer_result.is_empty(){
                break
            }else{
                run(buffer_result);
            }
        }else {
            break;
        }
    }
}

fn run(source: &str) -> ErrorReporter {
    // !TODO add handling of tokens
    println!("{}", source);

    ErrorReporter{
        had_error: false,
    }
}

