use std::{fs, process};
use std::fs::File;
use std::io::Write;
use clap::{Parser};
use anyhow::Result;
use crate::args::Args;

mod args;

struct GenerateAst {}

impl GenerateAst {
    fn define_ast(imports: Vec<&str>, base_name: &str, output_dir: &str, types: Vec<&str>) -> Result<()> {
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
        println!("Generated {} in file {}", base_name, file_path);
        Ok(())
    }

    fn define_type(file: &mut File, class_name: &str, fields: &str) -> Result<()> {
        // define struct
        file.write_all(format!("\n// {}", class_name).as_bytes())?;
        file.write_all(format!("\n#[derive(Debug, Clone, Eq, Hash, PartialEq)]\n").as_bytes())?;
        file.write_all(format!("pub struct {} {{\n", class_name).as_bytes())?;

        for field in fields.split(',') {
            let field_parts: Vec<&str> = field.trim().split_whitespace().collect();
            let field_type = field_parts[0];
            let field_name = field_parts[1];
            file.write_all(format!("    {}: {},\n", field_name, field_type).as_bytes())?;
        }
        file.write_all(format!("}}\n").as_bytes())?;

        // define implementation
        file.write_all(format!("\nimpl {} {{\n", class_name).as_bytes())?;

        // split fields to collection of tuples with field type and field name
        let field_list: Vec<(&str, &str)> = fields.split(',')
            .map(|field| {
                let parts: Vec<&str> = field.trim().split_whitespace().collect();
                (parts[0], parts[1])
            })
            .collect();

        // define constructor
        file.write_all(format!("    pub fn new(").as_bytes())?;
        for (i, (field_type, field_name)) in field_list.iter().enumerate() {
            if i > 0 {
                file.write_all(format!(", ").as_bytes())?;
            }
            file.write_all(format!("{}: {}", field_name, field_type).as_bytes())?;
        }
        file.write_all(format!(") -> Self {{\n").as_bytes())?;
        file.write_all(format!("        {} {{\n", class_name).as_bytes())?;
        for (_field_type, field_name) in &field_list {
            file.write_all(format!("            {},\n", field_name).as_bytes())?;
        }
        file.write_all(format!("        }}\n").as_bytes())?;
        file.write_all(format!("    }}\n").as_bytes())?;

        // define field accessors
        for (_field_type, field_name) in &field_list {
            file.write_all(format!("\n    pub fn {}(&self) -> &{} {{\n", field_name, _field_type).as_bytes())?;
            file.write_all(format!("        &self.{}\n", field_name).as_bytes())?;
            file.write_all(format!("    }}\n").as_bytes())?;
        }

        file.write_all(format!("}}\n").as_bytes())?;

        Ok(())
    }

    fn define_visitor(file: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {
        // define expression enum
        file.write_all(format!("\n// Expression enum").as_bytes())?;
        file.write_all(format!("\n#[derive(Debug, Clone, Hash, Eq, PartialEq)]\n").as_bytes())?;
        file.write_all(format!("pub enum {} {{\n", base_name).as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();

            file.write_all(format!("    {}({}),\n", type_name, type_name).as_bytes())?;
        }
        file.write_all(format!("}}\n").as_bytes())?;


        // define visitor trait
        file.write_all(format!("\n// Visitor trait").as_bytes())?;
        file.write_all(format!("\npub trait Visitor<T> {{\n").as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();
            file.write_all(format!("    fn visit_{}_expr(&mut self, expr: Rc<Expr>) -> Result<T>;\n", type_name.to_lowercase()).as_bytes())?;
        }
        file.write_all(format!("}}\n").as_bytes())?;

        // Implement `accept()`
        file.write_all(format!("\n// Implement accept for {}", base_name).as_bytes())?;
        file.write_all(format!("\nimpl {} {{\n", base_name).as_bytes())?;
        file.write_all(format!("    pub fn accept<T>({}: Rc<{}>, visitor: &mut dyn Visitor<T>) -> Result<T> {{\n", base_name.to_lowercase(), base_name).as_bytes())?;
        file.write_all(format!("        match {}.as_ref() {{\n", base_name.to_lowercase()).as_bytes())?;
        for t in types {
            let type_descr: Vec<&str> = t.split(':').collect();
            let type_name = type_descr[0].trim();
            file.write_all(format!("            {}::{}(_) => visitor.visit_{}_expr({}),\n",
                                  base_name, type_name, type_name.to_lowercase(), base_name.to_lowercase()).as_bytes())?;
        }
        file.write_all(format!("        }}\n").as_bytes())?;
        file.write_all(format!("    }}\n").as_bytes())?;
        file.write_all(format!("}}\n").as_bytes())?;

        Ok(())
    }
}

impl GenerateAst {
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Attempt to prepare output directories.
    match fs::create_dir_all(&args.output) {
        Ok(_val) => println!("Directories created."),
        Err(e) => {
            println!("Failed to create directories: {:?}", e);
            process::exit(1);
        }
    }

    let _ = GenerateAst::define_ast(
        vec!["use std::rc::Rc;\n\n",
                    "use crate::literal::Literal;\n",
                    "use crate::lox::Lox;\n",
                    "use crate::token::Token;\n",
                    "use crate::token_type::TokenType;\n"],
        "Expr",
        &args.output,
        vec![
            "Assign   : Token name, Rc<Expr> value",
            "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right",
            "Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments",
            "Get      : Rc<Expr> object, Token name",
            "Grouping : Rc<Expr> expression",
            "Literal  : LiteralValue value",
            "Logical  : Rc<Expr> left, Token operator, Rc<Expr> right",
            "Set      : Rc<Expr> object, Token name, Rc<Expr> value",
            "Super    : Token keyword, Token method",
            "This     : Token keyword",
            "Unary    : Token operator, Rc<Expr> right",
            "Variable : Token name"
        ]);

    let _ = GenerateAst::define_ast(
        vec!["use std::rc::Rc;\n\n",
             "use crate::literal::Literal;\n",
             "use crate::lox::Lox;\n",
             "use crate::token::Token;\n",
             "use crate::token_type::TokenType;\n"],
        "Stmt",
        &args.output,
        vec![
            "Block      : Vec<Rc<Stmt>> statements",
            "Class      : Token name, Option<Rc<Expr>> superclass, \
                                  Vec<Rc<Function>> methods",
            "Expression : Rc<Expr> expression",
            "Function   : Token name, Vec<Token> params, \
                                  Vec<Rc<Stmt>> body",
            "If         : Rc<Expr> condition, Rc<Stmt> then_branch, \
                                  Option<Rc<Stmt>> else_branch",
            "Print      : Rc<Expr> expression",
            "Return     : Token keyword, Option<Rc<Expr>> value",
            "Var        : Token name, Option<Rc<Expr>> initializer",
            "While      : Rc<Expr> condition, Rc<Stmt> body"
        ]);
    Ok(())
}
