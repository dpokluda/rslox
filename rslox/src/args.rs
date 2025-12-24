use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rslox")]
#[command(about = "Rust implementation of Lox language from Crafting Interpreters book.")]
pub struct Args {
    /// Script file to execute
    #[arg(short, long)]
    pub file: Option<String>,
    
    /// Run in interactive REPL mode
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,
}
