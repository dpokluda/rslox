//[ Appendix II stmt
use crate::literal::LiteralValue;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::expr::Expr;
use anyhow::Result;

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

// Expression enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<T, RuntimeError>;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<T, RuntimeError>;
}

// Implement accept for Stmt
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, RuntimeError> {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
        }
    }
}

//] Appendix II stmt
