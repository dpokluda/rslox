use crate::{cprint, cprintln, scanner};
use anyhow::Result;


pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }
    
    pub fn run_file(&mut self, file: &str) -> Result<()> {
        let contents = std::fs::read_to_string(file)?;
        self.run(contents.as_str())?;

        if self.had_error {
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

        self.had_error = false;
        
        Ok(())
    }

    fn run(&mut self, source: &str) -> anyhow::Result<()> {
        let scanner = scanner::Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }

    fn error(&mut self, line: u32, message: &str) -> anyhow::Result<()> {
        self.report(line, "", message);

        Ok(())
    }

    fn report(&mut self, line: u32, what: &str, message: &str) -> anyhow::Result<()> {
        cprintln!(colored::Color::Red, "[line {}] Error{}: {}", line, what, message);
        self.had_error = true;

        Ok(())
    }
}
