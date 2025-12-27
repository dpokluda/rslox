//[ Appendix II expr
use std::rc::Rc;

use crate::literal::Literal;
use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

// Assign
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Assign {
    name: Token,
    value: Rc<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Rc<Expr>) -> Self {
        Assign {
            name,
            value,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Rc<Expr> {
        &self.value
    }
}

// Binary
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Binary {
    left: Rc<Expr>,
    operator: Token,
    right: Rc<Expr>,
}

impl Binary {
    pub fn new(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Rc<Expr> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
        &self.right
    }
}

// Call
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Call {
    callee: Rc<Expr>,
    paren: Token,
    arguments: Vec<Rc<Expr>>,
}

impl Call {
    pub fn new(callee: Rc<Expr>, paren: Token, arguments: Vec<Rc<Expr>>) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }

    pub fn callee(&self) -> &Rc<Expr> {
        &self.callee
    }

    pub fn paren(&self) -> &Token {
        &self.paren
    }

    pub fn arguments(&self) -> &Vec<Rc<Expr>> {
        &self.arguments
    }
}

// Get
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Get {
    object: Rc<Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Rc<Expr>, name: Token) -> Self {
        Get {
            object,
            name,
        }
    }

    pub fn object(&self) -> &Rc<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}

// Grouping
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Grouping {
    expression: Rc<Expr>,
}

impl Grouping {
    pub fn new(expression: Rc<Expr>) -> Self {
        Grouping {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
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
    left: Rc<Expr>,
    operator: Token,
    right: Rc<Expr>,
}

impl Logical {
    pub fn new(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Rc<Expr> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
        &self.right
    }
}

// Set
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Set {
    object: Rc<Expr>,
    name: Token,
    value: Rc<Expr>,
}

impl Set {
    pub fn new(object: Rc<Expr>, name: Token, value: Rc<Expr>) -> Self {
        Set {
            object,
            name,
            value,
        }
    }

    pub fn object(&self) -> &Rc<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Rc<Expr> {
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
    right: Rc<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Rc<Expr>) -> Self {
        Unary {
            operator,
            right,
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
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
    fn visit_assign_expr(&mut self, assign: &Assign) -> Result<T>;
    fn visit_binary_expr(&mut self, binary: &Binary) -> Result<T>;
    fn visit_call_expr(&mut self, call: &Call) -> Result<T>;
    fn visit_get_expr(&mut self, get: &Get) -> Result<T>;
    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> Result<T>;
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<T>;
    fn visit_logical_expr(&mut self, logical: &Logical) -> Result<T>;
    fn visit_set_expr(&mut self, set: &Set) -> Result<T>;
    fn visit_super_expr(&mut self, super_: &Super) -> Result<T>;
    fn visit_this_expr(&mut self, this: &This) -> Result<T>;
    fn visit_unary_expr(&mut self, unary: &Unary) -> Result<T>;
    fn visit_variable_expr(&mut self, variable: &Variable) -> Result<T>;
}

// Implement accept for Expr
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T> {
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
