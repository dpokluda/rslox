use std::sync::atomic::{AtomicBool, Ordering};
use crate::{cprint, cprintln, scanner};
use anyhow::Result;


pub struct Lox {
}

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

impl Lox {
    pub fn new() -> Lox {
        HAD_ERROR.store(false, Ordering::SeqCst);
        Lox {}
    }

    pub fn run_file(&mut self, file: &str) -> Result<()> {
        let contents = std::fs::read_to_string(file)?;
        self.run(contents.as_str())?;

        if Lox::had_error() {
            std::process::exit(65);
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
        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }

    pub fn error(line: u32, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: u32, what: &str, message: &str) {
        cprintln!(colored::Color::Red, "[line {}] Error{}: {}", line, what, message);
        HAD_ERROR.store(true, Ordering::SeqCst);
    }

    pub fn had_error() -> bool {
        HAD_ERROR.load(Ordering::SeqCst)
    }
}
