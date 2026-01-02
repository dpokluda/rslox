use crate::expr::{Expr, Binary, Grouping, Literal, Unary, Variable, Assign, Logical};
use crate::{expr, lox, stmt};
use crate::runtime_error::RuntimeError;
use crate::stmt::{Block, Expression, If, Print, Stmt, Var, While};
use crate::value::Value;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    environment: Rc<RefCell<crate::environment::Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(crate::environment::Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {},
                Err(e) => lox::Lox::runtime_error(&e),
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &stmt::Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, statements: &Vec<Box<stmt::Stmt>>, environment: Rc<RefCell<crate::environment::Environment>>) -> Result<(), RuntimeError> {
        let previous = std::mem::replace(&mut self.environment, environment);

        let result = (|| {
            for statement in statements {
                self.execute(&statement)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        if a == &Value::Nil && b == &Value::Nil {
            true
        } else if a == &Value::Nil || b == &Value::Nil {
            false
        } else {
            a == b
        }
    }

    fn check_number_operand(&self, operator: &crate::token::Token, operand: &Value) -> Result<f64, RuntimeError> {
        if let Value::Number(n) = operand {
            Ok(*n)
        } else {
            Err(RuntimeError::new(
                operator.clone(),
                "Operand must be a number.".to_string(),
            ))
        }
    }
}

impl expr::Visitor<Value> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> anyhow::Result<Value, RuntimeError> {
        let value = self.evaluate(expr.value())?;
        self.environment.borrow_mut().assign(expr.name(), value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(&mut self, binary: &Binary) -> anyhow::Result<Value, RuntimeError> {
        let left = self.evaluate(binary.left())?;
        let right = self.evaluate(binary.right())?;
        match binary.operator().token_type() {
            crate::token::TokenType::Plus => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                    _ => Err(RuntimeError::new(
                        binary.operator().clone(),
                        "Operands must be two numbers or two strings.".to_string(),
                    )),
                }
            },
            crate::token::TokenType::Minus => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l - r))
            },
            crate::token::TokenType::Star => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l * r))
            },
            crate::token::TokenType::Slash => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l / r))
            },
            crate::token::TokenType::Greater => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l > r))
            },
            crate::token::TokenType::GreaterEqual => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l >= r))
            },
            crate::token::TokenType::Less => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l < r))
            },
            crate::token::TokenType::LessEqual => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l <= r))
            },
            crate::token::TokenType::EqualEqual => {
                Ok(Value::Boolean(self.is_equal(&left, &right)))
            },
            crate::token::TokenType::BangEqual => {
                Ok(Value::Boolean(!self.is_equal(&left, &right)))
            },
            _ => Err(RuntimeError::new(
                binary.operator().clone(),
                "Unknown binary operator.".to_string(),
            )),
        }
    }

    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> anyhow::Result<Value, RuntimeError> {
        self.evaluate(grouping.expression())
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> anyhow::Result<Value, RuntimeError> {
        match &literal.value() {
            crate::literal::LiteralValue::Number(n) => Ok(Value::Number(*n)),
            crate::literal::LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            crate::literal::LiteralValue::String(s) => Ok(Value::String(s.clone())),
            crate::literal::LiteralValue::Nil => Ok(Value::Nil),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> anyhow::Result<Value, RuntimeError> {
        let left = self.evaluate(expr.left())?;
        match expr.operator().token_type() {
            crate::token::TokenType::Or => {
                if self.is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(expr.right())
                }
            },
            crate::token::TokenType::And => {
                if !self.is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(expr.right())
                }
            },
            _ => Err(RuntimeError::new(
                expr.operator().clone(),
                "Unknown logical operator.".to_string(),
            )),
        }
    }

    fn visit_unary_expr(&mut self, unary: &Unary) -> anyhow::Result<Value, RuntimeError> {
        let right = self.evaluate(unary.right())?;
        match unary.operator().token_type() {
            crate::token::TokenType::Minus => {
                let n = self.check_number_operand(unary.operator(), &right)?;
                Ok(Value::Number(-n))
            },
            crate::token::TokenType::Bang => {
                Ok(Value::Boolean(!self.is_truthy(&right)))
            },
            _ => Err(RuntimeError::new(
                unary.operator().clone(),
                "Unknown unary operator.".to_string(),
            )),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> anyhow::Result<Value, RuntimeError> {
        self.environment.borrow().get(expr.name())
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> anyhow::Result<(), RuntimeError> {
        let new_environment = Rc::new(RefCell::new(crate::environment::Environment::from_enclosing(self.environment.clone())));
        self.execute_block(stmt.statements(), new_environment)?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> anyhow::Result<(), RuntimeError> {
        self.evaluate(stmt.statements())?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> anyhow::Result<(), RuntimeError> {
        let condition = self.evaluate(stmt.condition())?;
        if self.is_truthy(&condition) {
            self.execute(stmt.then_branch())?;
        } else if let Some(else_branch) = stmt.else_branch() {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> anyhow::Result<(), RuntimeError> {
        let value = self.evaluate(stmt.statements())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> anyhow::Result<(), RuntimeError> {
        let value = if let Some(initializer) = stmt.initializer() {
            self.evaluate(initializer)?
        } else {
            Value::Nil
        };
        self.environment.borrow_mut().define(stmt.name().lexeme().to_string(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> anyhow::Result<(), RuntimeError> {
        loop {
            let condition = self.evaluate(stmt.condition())?;
            if !self.is_truthy(&condition) {
                break;
            }
            self.execute(stmt.body())?;
        }
        Ok(())
    }
}
