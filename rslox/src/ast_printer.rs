use crate::expr::{Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable, Visitor};
use anyhow::Result;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self).unwrap()
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> Result<String> {
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

impl Visitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, assign: &Assign) -> Result<String> {
        todo!()     
    }

    fn visit_binary_expr(&mut self, binary: &Binary) -> Result<String> {
        self.parenthesize(&binary.operator().lexeme, &[binary.left(), binary.right()])
        
    }

    fn visit_call_expr(&mut self, call: &Call) -> Result<String> {
        todo!()
    }

    fn visit_get_expr(&mut self, get: &Get) -> Result<String> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> Result<String> {
        self.parenthesize("group", &[grouping.expression()])
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<String> {
        match &literal.value() {
            crate::literal::LiteralValue::Number(n) => Ok(n.to_string()),
            crate::literal::LiteralValue::String(s) => Ok(s.clone()),
            crate::literal::LiteralValue::Boolean(b) => Ok(b.to_string()),
            crate::literal::LiteralValue::Nil => Ok("nil".to_string()),
        }
    }

    fn visit_logical_expr(&mut self, logical: &Logical) -> Result<String> {
        todo!()
    }

    fn visit_set_expr(&mut self, set: &Set) -> Result<String> {
        todo!()
    }

    fn visit_super_expr(&mut self, super_: &Super) -> Result<String> {
        todo!()
    }

    fn visit_this_expr(&mut self, this: &This) -> Result<String> {
        todo!()
    }

    fn visit_unary_expr(&mut self, unary: &Unary) -> Result<String> {
        self.parenthesize(&unary.operator().lexeme, &[unary.right()])
    }

    fn visit_variable_expr(&mut self, variable: &Variable) -> Result<String> {
        todo!()
    }
}