use std::{fs, process};
use std::fs::File;
use std::io::Write;
use clap::Parser;
use anyhow::Result;
use crate::args::Args;

mod args;
mod console_macros;

struct GenerateAst {}

impl GenerateAst {
    fn define_ast(imports: Vec<&str>, base_name: &str, output_dir: &str, types: Vec<&str>) -> Result<()> {
        cprintln!(colored::Color::BrightYellow, "Generating AST for {}...", base_name);
        let file_path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
        let mut file = File::create(&file_path)?;

        file.write_all(format!("//[ Appendix II {}\n", base_name.to_lowercase()).as_bytes())?;

        for import in imports {
            file.write_all(import.as_bytes())?;
        }

        for t in &types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let class_name = type_descr[0].trim();
            let fields = type_descr[1].trim();

            Self::define_type(&mut file, class_name, fields)?;
        }

        Self::define_visitor(&mut file, base_name, &types)?;

        file.write_all(format!("\n//] Appendix II {}\n", base_name.to_lowercase()).as_bytes())?;
        println!("  Generated {} in file {}", base_name, file_path);
        Ok(())
    }

    fn define_type(file: &mut File, class_name: &str, fields: &str) -> Result<()> {
        // define struct
        file.write_all(format!("\n// {}", class_name).as_bytes())?;
        file.write_all("\n#[derive(Debug, Clone, Eq, Hash, PartialEq)]\n".as_bytes())?;
        file.write_all(format!("pub struct {} {{\n", class_name).as_bytes())?;

        for field in fields.split(',') {
            let field_parts: Vec<&str> = field.trim().split_whitespace().collect();
            let field_type = field_parts[0];
            let field_name = Self::safe_ident(field_parts[1]);
            file.write_all(format!("    {}: {},\n", field_name, field_type).as_bytes())?;
        }
        file.write_all("}\n".as_bytes())?;

        // define implementation
        file.write_all(format!("\nimpl {} {{\n", class_name).as_bytes())?;

        // split fields to collection of tuples with field type and field name
        let field_list: Vec<(&str, String)> = fields.split(',')
            .map(|field| {
                let parts: Vec<&str> = field.trim().split_whitespace().collect();
                (parts[0], Self::safe_ident(parts[1]))
            })
            .collect();

        // define constructor
        file.write_all("    pub fn new(".as_bytes())?;
        for (i, (field_type, field_name)) in field_list.iter().enumerate() {
            if i > 0 {
                file.write_all(", ".as_bytes())?;
            }
            file.write_all(format!("{}: {}", field_name, field_type).as_bytes())?;
        }
        file.write_all(") -> Self {\n".as_bytes())?;
        file.write_all(format!("        {} {{\n", class_name).as_bytes())?;
        for (_field_type, field_name) in &field_list {
            file.write_all(format!("            {},\n", field_name).as_bytes())?;
        }
        file.write_all("        }\n".as_bytes())?;
        file.write_all("    }\n".as_bytes())?;

        // define field accessors
        for (field_type, field_name) in &field_list {
            file.write_all(format!("\n    pub fn {}(&self) -> &{} {{\n", field_name, field_type).as_bytes())?;
            file.write_all(format!("        &self.{}\n", field_name).as_bytes())?;
            file.write_all("    }\n".as_bytes())?;
        }

        file.write_all("}\n".as_bytes())?;

        Ok(())
    }

    fn define_visitor(file: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {
        // define expression enum
        file.write_all("\n// Expression enum".as_bytes())?;
        file.write_all("\n#[derive(Debug, Clone, Hash, Eq, PartialEq)]\n".as_bytes())?;
        file.write_all(format!("pub enum {} {{\n", base_name).as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();

            file.write_all(format!("    {}({}),\n", type_name, type_name).as_bytes())?;
        }
        file.write_all("}\n".as_bytes())?;

        // define visitor trait
        file.write_all("\n// Visitor trait".as_bytes())?;
        file.write_all("\npub trait Visitor<T> {\n".as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();
            let safe_type_name = Self::safe_ident(&type_name.to_lowercase());
            file.write_all(format!(
                "    fn visit_{}_{}(&mut self, {}: &{}) -> Result<T, RuntimeError>;\n",
                type_name.to_lowercase(), base_name.to_lowercase(), base_name.to_lowercase(), type_name
            ).as_bytes())?;
        }
        file.write_all("}\n".as_bytes())?;

        // Implement `accept()`
        file.write_all(format!("\n// Implement accept for {}", base_name).as_bytes())?;
        file.write_all(format!("\nimpl {} {{\n", base_name).as_bytes())?;
        file.write_all("    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, RuntimeError> {\n".as_bytes())?;
        file.write_all("        match self {\n".as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();
            let safe_type_name = Self::safe_ident(&type_name.to_lowercase());
            file.write_all(format!(
                "            {}::{}({}) => visitor.visit_{}_{}({}),\n",
                base_name, type_name, base_name.to_lowercase(), type_name.to_lowercase(), base_name.to_lowercase(), base_name.to_lowercase()
            ).as_bytes())?;
        }
        file.write_all("        }\n".as_bytes())?;
        file.write_all("    }\n".as_bytes())?;
        file.write_all("}\n".as_bytes())?;

        Ok(())
    }

    /// Returns a safe Rust identifier, appending an underscore if the name is a keyword.
    fn safe_ident(name: &str) -> String {
        const KEYWORDS: &[&str] = &[
            "as", "break", "const", "continue", "crate", "else", "enum", "extern",
            "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
            "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
            "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
            "while", "async", "await", "dyn", "abstract", "become", "box", "do",
            "final", "macro", "override", "priv", "try", "typeof", "unsized",
            "virtual", "yield"
        ];
        if KEYWORDS.contains(&name) {
            format!("{}_", name)
        } else {
            name.to_string()
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Attempt to prepare output directories.
    match fs::create_dir_all(&args.output) {
        Ok(_) => println!("Directories created."),
        Err(e) => {
            println!("Failed to create directories: {:?}", e);
            process::exit(1);
        }
    }

    let _ = GenerateAst::define_ast(
        vec!["use crate::literal::LiteralValue;\n",
             "use crate::runtime_error::RuntimeError;\n",
             "use crate::token::Token;\n",
             "use anyhow::Result;\n"],
        "Expr",
        &args.output,
        vec![
            // "Assign   : Token name, Box<Expr> value",
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right",
            // "Call     : Box<Expr> callee, Token paren, Vec<Box<Expr>> arguments",
            // "Get      : Box<Expr> object, Token name",
            "Grouping : Box<Expr> expression",
            "Literal  : LiteralValue value",
            // "Logical  : Box<Expr> left, Token operator, Box<Expr> right",
            // "Set      : Box<Expr> object, Token name, Box<Expr> value",
            // "Super    : Token keyword, Token method",
            // "This     : Token keyword",
            "Unary    : Token operator, Box<Expr> right",
            // "Variable : Token name"
        ]);

        let _ = GenerateAst::define_ast(
        vec!["use crate::literal::LiteralValue;\n",
             "use crate::runtime_error::RuntimeError;\n",
             "use crate::token::Token;\n",
             "use crate::expr::Expr;\n",
             "use anyhow::Result;\n"],
        "Stmt",
        &args.output,
        vec![
            // "Block      : Vec<Box<Stmt>> statements",
            // "Class      : Token name, Option<Box<Expr>> superclass, Vec<Box<Function>> methods",
            "Expression : Box<Expr> statements",
            // "Function   : Token name, Vec<Token> params, Vec<Box<Stmt>> body",
            // "If         : Box<Expr> condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch",
            "Print      : Box<Expr> statements",
            // "Return     : Token keyword, Option<Box<Expr>> value",
            // "Var        : Token name, Option<Box<Expr>> initializer",
            // "While      : Box<Expr> condition, Box<Stmt> body"
        ]);
    
    cprintln!(colored::Color::Green, "Finished.");
    Ok(())
}
