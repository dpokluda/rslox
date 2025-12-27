use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "generate_ast")]
#[command(about = "Generates AST classes for the Lox interpreter", long_about = None)]
pub struct Args {
    /// Path to the output directory
    #[arg(short, long)]
    pub output: String,
}
