//[ Appendix II stmt
use std::rc::Rc;

use crate::literal::Literal;
use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

// Block
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Block {
    statements: Vec<Rc<Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Rc<Stmt>>) -> Self {
        Block {
            statements,
        }
    }

    pub fn statements(&self) -> &Vec<Rc<Stmt>> {
        &self.statements
    }
}

// Class
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Class {
    name: Token,
    superclass: Option<Rc<Expr>>,
    methods: Vec<Rc<Function>>,
}

impl Class {
    pub fn new(name: Token, superclass: Option<Rc<Expr>>, methods: Vec<Rc<Function>>) -> Self {
        Class {
            name,
            superclass,
            methods,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn superclass(&self) -> &Option<Rc<Expr>> {
        &self.superclass
    }

    pub fn methods(&self) -> &Vec<Rc<Function>> {
        &self.methods
    }
}

// Expression
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Expression {
    expression: Rc<Expr>,
}

impl Expression {
    pub fn new(expression: Rc<Expr>) -> Self {
        Expression {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
        &self.expression
    }
}

// Function
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Rc<Stmt>>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>) -> Self {
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

    pub fn body(&self) -> &Vec<Rc<Stmt>> {
        &self.body
    }
}

// If
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct If {
    condition: Rc<Expr>,
    then_branch: Rc<Stmt>,
    else_branch: Option<Rc<Stmt>>,
}

impl If {
    pub fn new(condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: Option<Rc<Stmt>>) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn condition(&self) -> &Rc<Expr> {
        &self.condition
    }

    pub fn then_branch(&self) -> &Rc<Stmt> {
        &self.then_branch
    }

    pub fn else_branch(&self) -> &Option<Rc<Stmt>> {
        &self.else_branch
    }
}

// Print
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Print {
    expression: Rc<Expr>,
}

impl Print {
    pub fn new(expression: Rc<Expr>) -> Self {
        Print {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
        &self.expression
    }
}

// Return
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Return {
    keyword: Token,
    value: Option<Rc<Expr>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Rc<Expr>>) -> Self {
        Return {
            keyword,
            value,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn value(&self) -> &Option<Rc<Expr>> {
        &self.value
    }
}

// Var
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Var {
    name: Token,
    initializer: Option<Rc<Expr>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Rc<Expr>>) -> Self {
        Var {
            name,
            initializer,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Option<Rc<Expr>> {
        &self.initializer
    }
}

// While
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct While {
    condition: Rc<Expr>,
    body: Rc<Stmt>,
}

impl While {
    pub fn new(condition: Rc<Expr>, body: Rc<Stmt>) -> Self {
        While {
            condition,
            body,
        }
    }

    pub fn condition(&self) -> &Rc<Expr> {
        &self.condition
    }

    pub fn body(&self) -> &Rc<Stmt> {
        &self.body
    }
}

// Expression enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Stmt {
    Block(Block),
    Class(Class),
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
    fn visit_block_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_class_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_expression_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_function_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_if_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_print_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_return_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_var_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
    fn visit_while_expr(&mut self, expr: Rc<Expr>) -> Result<T>;
}

// Implement accept for Stmt
impl Stmt {
    pub fn accept<T>(stmt: Rc<Stmt>, visitor: &mut dyn Visitor<T>) -> Result<T> {
        match stmt.as_ref() {
            Stmt::Block(_) => visitor.visit_block_expr(stmt),
            Stmt::Class(_) => visitor.visit_class_expr(stmt),
            Stmt::Expression(_) => visitor.visit_expression_expr(stmt),
            Stmt::Function(_) => visitor.visit_function_expr(stmt),
            Stmt::If(_) => visitor.visit_if_expr(stmt),
            Stmt::Print(_) => visitor.visit_print_expr(stmt),
            Stmt::Return(_) => visitor.visit_return_expr(stmt),
            Stmt::Var(_) => visitor.visit_var_expr(stmt),
            Stmt::While(_) => visitor.visit_while_expr(stmt),
        }
    }
}

//] Appendix II stmt
