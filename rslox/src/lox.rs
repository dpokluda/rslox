use std::sync::atomic::{AtomicBool, Ordering};
use crate::{cprint, cprintln, scanner};
use anyhow::Result;
use scanner::Scanner;
use crate::interpreter::Interpreter;
use crate::parse_error::ParseError;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::runtime_error::RuntimeError;

pub struct Lox {
    interpreter: Interpreter,
}

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

impl Lox {
    pub fn new() -> Lox {
        HAD_ERROR.store(false, Ordering::SeqCst);
        HAD_RUNTIME_ERROR.store(false, Ordering::SeqCst);
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, file: &str) -> Result<()> {
        let contents = std::fs::read_to_string(file)?;
        self.run(contents.as_str())?;

        if Lox::had_error() {
            std::process::exit(65);
        }
        if Lox::had_runtime_error() {
            std::process::exit(70);
        }
        
        Ok(())
    }

    pub fn run_prompt(&mut self) -> anyhow::Result<()> {
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

            self.run(line.as_str())?;
        }

        HAD_ERROR.store(false, Ordering::SeqCst);

        Ok(())
    }

    fn run(&mut self, source: &str) -> anyhow::Result<()> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        if Lox::had_error() {
            return Ok(());
        }

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve(&statements.clone().unwrap()).unwrap();

        if Lox::had_error() {
            return Ok(());
        }

        self.interpreter.interpret(&statements.clone().unwrap());

        Ok(())
    }

    pub fn error_line(line: u32, message: &str) {
        Lox::report(line, "", message);
    }

    pub fn parse_error(parse_error: &ParseError) {
        if parse_error.token().token_type() == &crate::token_type::TokenType::Eof {
            Lox::report(parse_error.token().line(), " at end", parse_error.message());
        } else {
            Lox::report(parse_error.token().line(), &format!(" at '{}'", parse_error.token().lexeme()), parse_error.message());
        }

    }

    pub fn runtime_error(runtime_error: &RuntimeError) {
        cprintln!(colored::Color::Red, "{}\n[line {}]", runtime_error.message(), runtime_error.token().line());
        HAD_RUNTIME_ERROR.store(true, Ordering::SeqCst);
    }

    fn report(line: u32, what: &str, message: &str) {
        cprintln!(colored::Color::Red, "[line {}] Error{}: {}", line, what, message);
        HAD_ERROR.store(true, Ordering::SeqCst);
    }

    pub fn had_error() -> bool {
        HAD_ERROR.load(Ordering::SeqCst)
    }
    
    pub fn had_runtime_error() -> bool {
        HAD_RUNTIME_ERROR.load(Ordering::SeqCst)
    }
}
