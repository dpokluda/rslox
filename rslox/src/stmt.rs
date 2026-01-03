//[ Appendix II stmt
use crate::runtime_error::LoxRuntime;
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

// Function
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<Stmt>>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Box<Stmt>>) -> Self {
        Function {
            name,
            params,
            body,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn body(&self) -> &Vec<Box<Stmt>> {
        &self.body
    }
}

// If
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct If {
    condition: Box<Expr>,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
}

impl If {
    pub fn new(condition: Box<Expr>, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn condition(&self) -> &Box<Expr> {
        &self.condition
    }

    pub fn then_branch(&self) -> &Box<Stmt> {
        &self.then_branch
    }

    pub fn else_branch(&self) -> &Option<Box<Stmt>> {
        &self.else_branch
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

// Return
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Return {
    keyword: Token,
    value: Option<Box<Expr>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Box<Expr>>) -> Self {
        Return {
            keyword,
            value,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn value(&self) -> &Option<Box<Expr>> {
        &self.value
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

// While
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct While {
    condition: Box<Expr>,
    body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
        While {
            condition,
            body,
        }
    }

    pub fn condition(&self) -> &Box<Expr> {
        &self.condition
    }

    pub fn body(&self) -> &Box<Stmt> {
        &self.body
    }
}

// Expression enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}

// Visitor trait
pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<T, LoxRuntime>;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<T, LoxRuntime>;
    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<T, LoxRuntime>;
    fn visit_if_stmt(&mut self, stmt: &If) -> Result<T, LoxRuntime>;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<T, LoxRuntime>;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<T, LoxRuntime>;
    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<T, LoxRuntime>;
    fn visit_while_stmt(&mut self, stmt: &While) -> Result<T, LoxRuntime>;
}

// Implement accept for Stmt
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, LoxRuntime> {
        match self {
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
        }
    }
}

//] Appendix II stmt
