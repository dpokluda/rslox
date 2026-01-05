mod args;
mod console_macros;
mod scanner;
mod token;
mod lox;
mod token_type;
mod literal;
mod parser;
mod expr;
mod stmt;
mod interpreter;
mod value;
mod parse_error;
mod runtime_error;
mod environment;
mod lox_callable;
mod lox_clock;
mod lox_function;
mod resolver;
mod macros;
mod lox_class;
mod lox_instance;

use clap::{CommandFactory, Parser};
use anyhow::Result;
use crate::args::Args;
use crate::lox::Lox;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut lox = Lox::new();

    if args.interactive {
        cprintln!(colored::Color::Cyan, "Running in interactive REPL mode...");
        lox.run_prompt()?;
    } else if let Some(filename) = args.file {
        cprintln!(colored::Color::Cyan, "Running script from file: {}", filename);
        lox.run_file(&filename)?;
    }
    else {
        cprintln!(colored::Color::Red, "No input provided. Use --help for usage information.");
        Args::command().print_help()?;
    }

    println!();
    cprintln!(colored::Color::Green, "Finished.");

    Ok(())
}
