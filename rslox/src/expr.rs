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

// Super
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Super {
            keyword,
            method,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn method(&self) -> &Token {
        &self.method
    }
}

// This
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct This {
    keyword: Token,
}

impl This {
    pub fn new(keyword: Token) -> Self {
        This {
            keyword,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
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
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, assign: &Assign) -> Result<T, RuntimeError>;
    fn visit_binary_expr(&mut self, binary: &Binary) -> Result<T, RuntimeError>;
    fn visit_call_expr(&mut self, call: &Call) -> Result<T, RuntimeError>;
    fn visit_get_expr(&mut self, get: &Get) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> Result<T, RuntimeError>;
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<T, RuntimeError>;
    fn visit_logical_expr(&mut self, logical: &Logical) -> Result<T, RuntimeError>;
    fn visit_set_expr(&mut self, set: &Set) -> Result<T, RuntimeError>;
    fn visit_super_expr(&mut self, super_: &Super) -> Result<T, RuntimeError>;
    fn visit_this_expr(&mut self, this: &This) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&mut self, unary: &Unary) -> Result<T, RuntimeError>;
    fn visit_variable_expr(&mut self, variable: &Variable) -> Result<T, RuntimeError>;
}

// Implement accept for Expr
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, RuntimeError> {
        match self {
            Expr::Assign(assign) => visitor.visit_assign_expr(assign),
            Expr::Binary(binary) => visitor.visit_binary_expr(binary),
            Expr::Call(call) => visitor.visit_call_expr(call),
            Expr::Get(get) => visitor.visit_get_expr(get),
            Expr::Grouping(grouping) => visitor.visit_grouping_expr(grouping),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Logical(logical) => visitor.visit_logical_expr(logical),
            Expr::Set(set) => visitor.visit_set_expr(set),
            Expr::Super(super_) => visitor.visit_super_expr(super_),
            Expr::This(this) => visitor.visit_this_expr(this),
            Expr::Unary(unary) => visitor.visit_unary_expr(unary),
            Expr::Variable(variable) => visitor.visit_variable_expr(variable),
        }
    }
}

//] Appendix II expr
