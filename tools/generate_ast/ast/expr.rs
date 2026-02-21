//[ Appendix II expr
use crate::literal::LiteralValue;
use crate::runtime_error::LoxRuntime;
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

// Call
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Box<Expr>>,
}

impl Call {
    pub fn new(callee: Box<Expr>, paren: Token, arguments: Vec<Box<Expr>>) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }

    pub fn callee(&self) -> &Box<Expr> {
        &self.callee
    }

    pub fn paren(&self) -> &Token {
        &self.paren
    }

    pub fn arguments(&self) -> &Vec<Box<Expr>> {
        &self.arguments
    }
}

// Get
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Box<Expr>, name: Token) -> Self {
        Get {
            object,
            name,
        }
    }

    pub fn object(&self) -> &Box<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
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

// Logical
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Logical {
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

// Set
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}

impl Set {
    pub fn new(object: Box<Expr>, name: Token, value: Box<Expr>) -> Self {
        Set {
            object,
            name,
            value,
        }
    }

    pub fn object(&self) -> &Box<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Box<Expr> {
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
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Unary(Unary),
    Variable(Variable),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<T, LoxRuntime>;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<T, LoxRuntime>;
    fn visit_call_expr(&mut self, expr: &Call) -> Result<T, LoxRuntime>;
    fn visit_get_expr(&mut self, expr: &Get) -> Result<T, LoxRuntime>;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<T, LoxRuntime>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<T, LoxRuntime>;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<T, LoxRuntime>;
    fn visit_set_expr(&mut self, expr: &Set) -> Result<T, LoxRuntime>;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<T, LoxRuntime>;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<T, LoxRuntime>;
}

// Implement accept for Expr
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, LoxRuntime> {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Call(expr) => visitor.visit_call_expr(expr),
            Expr::Get(expr) => visitor.visit_get_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
            Expr::Set(expr) => visitor.visit_set_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
        }
    }
}

//] Appendix II expr
