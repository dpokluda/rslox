use std::collections::HashMap;
use crate::{expr, stmt};
use crate::expr::{Assign, Binary, Call, Get, Grouping, Literal, Logical, Set, Unary, Variable};
use crate::interpreter::Interpreter;
use crate::runtime_error::{LoxRuntime, RuntimeError};
use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, stmts: &Vec<Box<Stmt>>) -> anyhow::Result<(), LoxRuntime> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &stmt::Stmt) -> anyhow::Result<(), LoxRuntime> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &expr::Expr) -> anyhow::Result<(), LoxRuntime> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<(), LoxRuntime> {
        if !self.scopes.is_empty() {
            if self.scopes.last().unwrap().contains_key(name.lexeme()) {
                return Err(LoxRuntime::Error(RuntimeError::new(
                    name.clone(),
                    "Variable with this name already declared in this scope.".to_string(),
                )));
            }
            self.scopes.last_mut().unwrap().insert(name.lexeme().to_string(), false);
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme().to_string(), true);
        }
    }

    fn resolve_local(&mut self, expr: &expr::Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.lexeme()) {
                let depth = self.scopes.len() - 1 - i;
                self.interpreter.resolve(expr, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &Function, func_type: FunctionType) -> anyhow::Result<(), LoxRuntime> {
        let enclosing_function = self.current_function;
        self.current_function = func_type;

        self.begin_scope();
        for param in function.params() {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(function.body())?;
        self.end_scope();

        self.current_function = enclosing_function;
        Ok(())
    }
}

impl<'a> expr::Visitor<()> for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.value())?;
        self.resolve_local(&expr::Expr::Assign(expr.clone()), expr.name());
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())?;
        Ok(())
    }

    fn visit_call_expr(&mut self, expr: &Call) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.callee())?;
        for argument in expr.arguments() {
            self.resolve_expr(argument)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, expr: &Get) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.object())?;
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.expression())?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> anyhow::Result<(), LoxRuntime> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())?;
        Ok(())
    }

    fn visit_set_expr(&mut self, expr: &Set) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.value())?;
        self.resolve_expr(expr.object())?;
        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(expr.right())?;
        Ok(())
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> anyhow::Result<(), LoxRuntime> {
        if let Some(scope) = self.scopes.last() {
            if let Some(false) = scope.get(expr.name().lexeme()) {
                return Err(LoxRuntime::Error(RuntimeError::new(
                    expr.name().clone(),
                    "Cannot read local variable in its own initializer.".to_string(),
                )));
            }
        }
        self.resolve_local(&expr::Expr::Variable(expr.clone()), expr.name());

        Ok(())
    }
}

impl<'a> stmt::Visitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> anyhow::Result<(), LoxRuntime> {
        self.begin_scope();
        self.resolve(stmt.statements())?;
        self.end_scope();
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> anyhow::Result<(), LoxRuntime> {
        self.declare(stmt.name())?;
        self.define(stmt.name());
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(stmt.expression())?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> anyhow::Result<(), LoxRuntime> {
        self.declare(stmt.name())?;
        self.define(stmt.name());
        
        self.resolve_function(stmt, FunctionType::Function)?;
        
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.then_branch())?;
        if let Some(else_branch) = &stmt.else_branch() {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(stmt.expression())?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> anyhow::Result<(), LoxRuntime> {
        if self.current_function == FunctionType::None {
            return Err(LoxRuntime::Error(RuntimeError::new(
                stmt.keyword().clone(),
                "Cannot return from top-level code.".to_string(),
            )));
        }
        
        if let Some(value) = stmt.value() {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> anyhow::Result<(), LoxRuntime> {
        self.declare(stmt.name())?;
        if let Some(initializer) = stmt.initializer() {
            self.resolve_expr(initializer)?;
        }
        self.define(stmt.name());
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> anyhow::Result<(), LoxRuntime> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.body())?;
        Ok(())
    }
}