mod args;
mod console_macros;
mod scanner;
mod token;

use clap::{CommandFactory, Parser};
use crate::args::Args;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.interactive {
        cprintln!(colored::Color::Cyan, "Running in interactive REPL mode...");
        run_prompt()?;
    } else if let Some(filename) = args.file {
        cprintln!(colored::Color::Cyan, "Running script from file: {}", filename);
        run_file(&filename)?;
    }
    else {
        cprintln!(colored::Color::Red, "No input provided. Use --help for usage information.");
        Args::command().print_help()?;
    }

    cprintln!(colored::Color::Green, "Finished.");

    Ok(())
}

fn run_file(file: &str) -> Result<()> {
    let contents = std::fs::read_to_string(file)?;
    run(contents.as_str())?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    use std::io::{self, Write};

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        cprint!(colored::Color::Yellow, "> ");
        stdout.flush()?;

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line)?;

        if bytes_read == 0 || line.trim().is_empty() {
            break; // EOF reached
        }

        run(line.as_str())?;
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let scanner = scanner::Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}