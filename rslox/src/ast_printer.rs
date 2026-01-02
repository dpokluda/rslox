use crate::expr::*;
use anyhow::Result;
use crate::expr;
use crate::runtime_error::RuntimeError;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self).unwrap()
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> Result<String, RuntimeError> {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self)?);
        }
        result.push(')');
        
        Ok(result)
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<String, RuntimeError> {
        todo!()
    }

    fn visit_binary_expr(&mut self, binary: &Binary) -> Result<String, RuntimeError> {
        self.parenthesize(&binary.operator().lexeme(), &[binary.left(), binary.right()])
        
    }

    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> Result<String, RuntimeError> {
        self.parenthesize("group", &[grouping.expression()])
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<String, RuntimeError> {
        match &literal.value() {
            crate::literal::LiteralValue::Number(n) => Ok(n.to_string()),
            crate::literal::LiteralValue::String(s) => Ok(s.clone()),
            crate::literal::LiteralValue::Boolean(b) => Ok(b.to_string()),
            crate::literal::LiteralValue::Nil => Ok("nil".to_string()),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<String, RuntimeError> {
        todo!()
    }

    fn visit_unary_expr(&mut self, unary: &Unary) -> Result<String, RuntimeError> {
        self.parenthesize(&unary.operator().lexeme(), &[unary.right()])
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<String, RuntimeError> {
        todo!()
    }
}