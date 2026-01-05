use crate::expr::{Expr, Binary, Grouping, Literal, Unary, Variable, Assign, Logical, Call, Get, Set};
use crate::{expr, stmt};
use crate::runtime_error::{LoxRuntime, RuntimeError, RuntimeReturn};
use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::value::Value;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::environment::Environment;
use crate::literal::LiteralValue;
use crate::lox::Lox;
use crate::token::{Token, TokenType};

#[derive(Clone)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    global: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Environment::new()));

        global.borrow_mut().define(
            "clock".to_string(),
            Value::LoxCallable(Rc::new(crate::lox_clock::LoxClock::new())),
        );

        Interpreter {
            environment: global.clone(),
            global,
            locals: HashMap::new(),
        }
    }

    pub fn globals(&self) -> Rc<RefCell<Environment>> {
        Rc::clone(&self.global)
    }

    pub fn interpret(&mut self, statements: &Vec<Box<Stmt>>) {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {},
                Err(e) => {
                    match e {
                        LoxRuntime::Error(runtime_error) => {
                            Lox::runtime_error(&runtime_error);
                        },
                        LoxRuntime::Return(_) => {
                            // This should never happen at the top level.
                            panic!("Unexpected return statement at top level.");
                        },
                    }
                }
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxRuntime> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxRuntime> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    pub fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, environment: Rc<RefCell<Environment>>) -> Result<(), LoxRuntime> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        
        let result = (|| {
            for statement in statements {
                self.execute(&statement)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    fn lookup_variable(&self, name: &Token, expr: &Expr) -> Result<Value, LoxRuntime> {
        if let Some(distance) = self.locals.get(expr) {
            self.environment.borrow().get_at(*distance, name.lexeme())
        } else {
            self.global.borrow().get(name)
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            _ => false,
        }
    }

    fn check_number_operand(&self, operator: &Token, operand: &Value) -> Result<f64, LoxRuntime> {
        if let Value::Number(n) = operand {
            Ok(*n)
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(
                operator.clone(),
                "Operand must be a number.".to_string(),
            )))
        }
    }
}

impl expr::Visitor<Value> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> anyhow::Result<Value, LoxRuntime> {
        let value = self.evaluate(expr.value())?;
        if let Some(distance) = self.locals.get(&expr::Expr::Assign(expr.clone())) {
            self.environment.borrow_mut().assign_at(*distance, expr.name(), value.clone())?;
        } else {
            self.global.borrow_mut().assign(expr.name(), value.clone())?;
        }
        Ok(value)
    }

    fn visit_binary_expr(&mut self, binary: &Binary) -> anyhow::Result<Value, LoxRuntime> {
        let left = self.evaluate(binary.left())?;
        let right = self.evaluate(binary.right())?;
        match binary.operator().token_type() {
            TokenType::Plus => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                    _ => Err(LoxRuntime::Error(RuntimeError::new(
                        binary.operator().clone(),
                        "Operands must be two numbers or two strings.".to_string(),
                    ))),
                }
            },
            TokenType::Minus => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l - r))
            },
            TokenType::Star => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l * r))
            },
            TokenType::Slash => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Number(l / r))
            },
            TokenType::Greater => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l > r))
            },
            TokenType::GreaterEqual => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l >= r))
            },
            TokenType::Less => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l < r))
            },
            TokenType::LessEqual => {
                let l = self.check_number_operand(binary.operator(), &left)?;
                let r = self.check_number_operand(binary.operator(), &right)?;
                Ok(Value::Boolean(l <= r))
            },
            TokenType::EqualEqual => {
                Ok(Value::Boolean(self.is_equal(&left, &right)))
            },
            TokenType::BangEqual => {
                Ok(Value::Boolean(!self.is_equal(&left, &right)))
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                binary.operator().clone(),
                "Unknown binary operator.".to_string(),
            ))),
        }
    }

    fn visit_call_expr(&mut self, expr: &Call) -> anyhow::Result<Value, LoxRuntime> {
        let callee = self.evaluate(expr.callee())?;

        let mut arguments = Vec::new();
        for argument in expr.arguments() {
            arguments.push(self.evaluate(argument)?);
        }

        match callee {
            Value::LoxCallable(function) => {
                if arguments.len() != function.arity() {
                    return Err(LoxRuntime::Error(RuntimeError::new(
                        expr.paren().clone(),
                        format!("Expected {} arguments but got {}.", function.arity(), arguments.len()),
                    )));
                }
                function.call(self, arguments)
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                expr.paren().clone(),
                "Can only call functions and classes.".to_string(),
            ))),
        }
    }

    fn visit_get_expr(&mut self, expr: &Get) -> anyhow::Result<Value, LoxRuntime> {
        let object = self.evaluate(expr.object())?;
        match object {
            Value::LoxInstance(instance) => {
                instance.borrow().get(expr.name())
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                expr.name().clone(),
                "Only instances have properties.".to_string(),
            ))),
        }
    }

    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> anyhow::Result<Value, LoxRuntime> {
        self.evaluate(grouping.expression())
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> anyhow::Result<Value, LoxRuntime> {
        match &literal.value() {
            LiteralValue::Number(n) => Ok(Value::Number(*n)),
            LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::Nil => Ok(Value::Nil),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> anyhow::Result<Value, LoxRuntime> {
        let left = self.evaluate(expr.left())?;
        match expr.operator().token_type() {
            TokenType::Or => {
                if self.is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(expr.right())
                }
            },
            TokenType::And => {
                if !self.is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(expr.right())
                }
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                expr.operator().clone(),
                "Unknown logical operator.".to_string(),
            ))),
        }
    }

    fn visit_set_expr(&mut self, expr: &Set) -> anyhow::Result<Value, LoxRuntime> {
        let object = self.evaluate(expr.object())?;
        match object {
            Value::LoxInstance(instance) => {
                let value = self.evaluate(expr.value())?;
                instance.borrow_mut().set(expr.name(), value.clone());
                Ok(value)
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                expr.name().clone(),
                "Only instances have fields.".to_string(),
            ))),
        }
    }

    fn visit_unary_expr(&mut self, unary: &Unary) -> anyhow::Result<Value, LoxRuntime> {
        let right = self.evaluate(unary.right())?;
        match unary.operator().token_type() {
            TokenType::Minus => {
                let n = self.check_number_operand(unary.operator(), &right)?;
                Ok(Value::Number(-n))
            },
            TokenType::Bang => {
                Ok(Value::Boolean(!self.is_truthy(&right)))
            },
            _ => Err(LoxRuntime::Error(RuntimeError::new(
                unary.operator().clone(),
                "Unknown unary operator.".to_string(),
            ))),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> anyhow::Result<Value, LoxRuntime> {
        self.lookup_variable(expr.name(), &expr::Expr::Variable(expr.clone()))
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> anyhow::Result<(), LoxRuntime> {
        let new_environment = Rc::new(RefCell::new(Environment::from_enclosing(self.environment.clone())));
        self.execute_block(stmt.statements(), new_environment)?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> anyhow::Result<(), LoxRuntime> {
        self.environment.borrow_mut().define(stmt.name().lexeme().to_string(), Value::Nil);
        let class_ = crate::lox_class::LoxClass::new(stmt.name().lexeme().to_string());
        self.environment.borrow_mut().assign(stmt.name(), Value::LoxClass(Rc::new(class_)))?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> anyhow::Result<(), LoxRuntime> {
        self.evaluate(stmt.expression())?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> anyhow::Result<(), LoxRuntime> {
        let function = crate::lox_function::LoxFunction::new(
            Box::new(stmt.clone()),
            self.environment.clone(),
        );
        self.environment.borrow_mut().define(
            stmt.name().lexeme().to_string(),
            Value::LoxCallable(Rc::new(function)),
        );
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> anyhow::Result<(), LoxRuntime> {
        let condition = self.evaluate(stmt.condition())?;
        if self.is_truthy(&condition) {
            self.execute(stmt.then_branch())?;
        } else if let Some(else_branch) = stmt.else_branch() {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> anyhow::Result<(), LoxRuntime> {
        let value = self.evaluate(stmt.expression())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> anyhow::Result<(), LoxRuntime> {
        let value = if let Some(expr) = stmt.value() {
            self.evaluate(expr)?
        } else {
            Value::Nil
        };
        Err(LoxRuntime::Return(RuntimeReturn::new(value)))
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> anyhow::Result<(), LoxRuntime> {
        let value = if let Some(initializer) = stmt.initializer() {
            self.evaluate(initializer)?
        } else {
            Value::Nil
        };
        self.environment.borrow_mut().define(stmt.name().lexeme().to_string(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> anyhow::Result<(), LoxRuntime> {
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
