use std::sync::atomic::{AtomicBool, Ordering};
use crate::{cprint, cprintln, scanner};
use anyhow::Result;
use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;

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
        let mut scanner = scanner::Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        let mut parser = crate::parser::Parser::new(tokens);
        let expr = parser.parse();

        if Lox::had_error() {
            return Ok(());
        }

        let mut printer = AstPrinter {};
        println!("{}", printer.print(&expr.clone().unwrap()));
        
        self.interpreter.interpret(&expr.unwrap());

        Ok(())
    }

    pub fn error_line(line: u32, message: &str) {
        Lox::report(line, "", message);
    }

    pub fn error_token(token: &crate::token::Token, message: &str) {
        if token.token_type() == &crate::token_type::TokenType::Eof {
            Lox::report(token.line(), " at end", message);
        } else {
            Lox::report(token.line(), &format!(" at '{}'", token.lexeme()), message);
        }

    }

    pub fn runtime_error(token: &crate::token::Token, message: &str) {
        cprintln!(colored::Color::Red, "{}\n[line {}]", message, token.line());
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
