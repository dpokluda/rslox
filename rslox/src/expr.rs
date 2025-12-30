//[ Appendix II expr
use crate::literal::LiteralValue;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use anyhow::Result;

// Assign
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Box<Expr>) -> Self {
        Assign {
            name,
            value,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Box<Expr> {
        &self.value
    }
}

// Binary
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Box<Expr> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Box<Expr> {
        &self.right
    }
}

// Grouping
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Grouping {
            expression,
        }
    }

    pub fn expression(&self) -> &Box<Expr> {
        &self.expression
    }
}

// Literal
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Literal {
    value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal {
            value,
        }
    }

    pub fn value(&self) -> &LiteralValue {
        &self.value
    }
}

// Unary
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Unary {
            operator,
            right,
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Box<Expr> {
        &self.right
    }
}

// Variable
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Variable {
            name,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}

// Expression enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<T, RuntimeError>;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<T, RuntimeError>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<T, RuntimeError>;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<T, RuntimeError>;
}

// Implement accept for Expr
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, RuntimeError> {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
        }
    }
}

//] Appendix II expr
