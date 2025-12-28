use crate::expr::{Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable, Visitor};
use crate::lox;
use crate::runtime_error::RuntimeError;
use crate::value::Value;

pub struct Interpreter { }

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { }
    }

    pub fn interpret(&mut self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("{}", value),
            Err(error) => lox::Lox::runtime_error(error.token(), error.message()),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
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

impl Visitor<Value> for Interpreter {
    fn visit_assign_expr(&mut self, assign: &Assign) -> anyhow::Result<Value, RuntimeError> {
        todo!()
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

    fn visit_call_expr(&mut self, call: &Call) -> anyhow::Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_get_expr(&mut self, get: &Get) -> anyhow::Result<Value, RuntimeError> {
        todo!()
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

    fn visit_logical_expr(&mut self, logical: &Logical) -> anyhow::Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_set_expr(&mut self, set: &Set) -> anyhow::Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_super_expr(&mut self, super_: &Super) -> anyhow::Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_this_expr(&mut self, this: &This) -> anyhow::Result<Value, RuntimeError> {
        todo!()
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

    fn visit_variable_expr(&mut self, variable: &Variable) -> anyhow::Result<Value, RuntimeError> {
        todo!()
    }
}