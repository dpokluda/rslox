//[ Appendix II stmt
use crate::literal::LiteralValue;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::expr::Expr;
use anyhow::Result;

// Block
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Block {
    statements: Vec<Box<Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Box<Stmt>>) -> Self {
        Block {
            statements,
        }
    }

    pub fn statements(&self) -> &Vec<Box<Stmt>> {
        &self.statements
    }
}

// Expression
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Expression {
    statements: Box<Expr>,
}

impl Expression {
    pub fn new(statements: Box<Expr>) -> Self {
        Expression {
            statements,
        }
    }

    pub fn statements(&self) -> &Box<Expr> {
        &self.statements
    }
}

// Print
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Print {
    statements: Box<Expr>,
}

impl Print {
    pub fn new(statements: Box<Expr>) -> Self {
        Print {
            statements,
        }
    }

    pub fn statements(&self) -> &Box<Expr> {
        &self.statements
    }
}

// Var
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Var {
    name: Token,
    initializer: Option<Box<Expr>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Box<Expr>>) -> Self {
        Var {
            name,
            initializer,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Option<Box<Expr>> {
        &self.initializer
    }
}

// Expression enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<T, RuntimeError>;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<T, RuntimeError>;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<T, RuntimeError>;
    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<T, RuntimeError>;
}

// Implement accept for Stmt
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, RuntimeError> {
        match self {
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
        }
    }
}

//] Appendix II stmt
