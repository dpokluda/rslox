use std::ops::Deref;

// Expression Object Model
pub struct Literal {
    value: f32,
}

impl Literal {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

pub struct Addition {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Addition {
    pub fn new(left: Box<Expression>, right: Box<Expression>) -> Self {
        Self {
            left,
            right,
        }
    }
}

pub struct Subtraction {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Subtraction {
    pub fn new(left: Box<Expression>, right: Box<Expression>) -> Self {
        Self {
            left,
            right,
        }
    }
}

// Visitor Trait
pub trait Visitor<T> {
    fn visit_literal_expression(&self, literal: &Literal) -> T;
    fn visit_addition_expression(&self, addition: &Addition) -> T;
    fn visit_subtraction_expression(&self, subtraction: &Subtraction) -> T;
}

// Expression Enum
pub enum Expression{
    Literal(Literal),
    Addition(Addition),
    Subtraction(Subtraction),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expression::Literal(literal) => visitor.visit_literal_expression(literal),
            Expression::Addition(addition) => visitor.visit_addition_expression(addition),
            Expression::Subtraction(subtraction) => visitor.visit_subtraction_expression(subtraction),
        }
    }
}

// Conrete Visitors
pub struct ExpressionPrintingVisitor {}

impl Visitor<()> for ExpressionPrintingVisitor {
    fn visit_literal_expression(&self, literal: &Literal) -> () {
        print!("{}", literal.value);
    }

    fn visit_addition_expression(&self, addition: &Addition) -> () {
        print!("(");
        addition.left.accept(self);
        print!(" + ");
        addition.right.accept(self);
        print!(")");
    }

    fn visit_subtraction_expression(&self, subtraction: &Subtraction) -> () {
        print!("(");
        subtraction.left.accept(self);
        print!(" - ");
        subtraction.right.accept(self);
        print!(")");
    }
}

pub struct ExpressionEvaluatingVisitor {}

impl Visitor<f32> for ExpressionEvaluatingVisitor {
    fn visit_literal_expression(&self, literal: &Literal) -> f32 {
        literal.value
    }

    fn visit_addition_expression(&self, addition: &Addition) -> f32 {
        let left_value = addition.left.accept(self);
        let right_value = addition.right.accept(self);
        left_value + right_value
    }

    fn visit_subtraction_expression(&self, subtraction: &Subtraction) -> f32 {
        let left_value = subtraction.left.accept(self);
        let right_value = subtraction.right.accept(self);
        left_value - right_value
    }
}

// Main program
fn main() {
    let printer = ExpressionPrintingVisitor {};
    let evaluator = ExpressionEvaluatingVisitor {};

    // Emulate (1 + 2) + 3
    let expr: Box<Expression> = Box::new(Expression::Addition(Addition::new(
        Box::new(Expression::Addition(Addition::new(
            Box::new(Expression::Literal(Literal::new(1.0))),
            Box::new(Expression::Literal(Literal::new(2.0))),
        ))),
        Box::new(Expression::Literal(Literal::new(3.0))),
    )));

    Expression::accept(expr.deref(), &printer);
    let result = Expression::accept(expr.deref(), &evaluator);
    println!(" = {}", result);

    // Emulate 1 - 2 = -1
    let expr: Box<Expression> = Box::new(Expression::Subtraction(Subtraction::new(
        Box::new(Expression::Literal(Literal::new(1.0))),
        Box::new(Expression::Literal(Literal::new(2.0)))
    )));

    Expression::accept(expr.deref(), &printer);
    let result = Expression::accept(expr.deref(), &evaluator);
    println!(" = {}", result);

    // Emulate (1 - 2) + 8 = 7
    let expr: Box<Expression> = Box::new(Expression::Addition(Addition::new(
        Box::new(Expression::Subtraction(Subtraction::new(
            Box::new(Expression::Literal(Literal::new(1.0))),
            Box::new(Expression::Literal(Literal::new(2.0)))
        ))),
        Box::new(Expression::Literal(Literal::new(8.0)))
    )));

    Expression::accept(expr.deref(), &printer);
    let result = Expression::accept(expr.deref(), &evaluator);
    println!(" = {}", result);
}
