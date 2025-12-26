pub trait ExpressionVisitor {
    fn visit_literal_expression(&self, literal: &Literal);
    fn visit_addition_expression(&self, addition: &Addition);
    fn visit_subtraction_expression(&self, subtraction: &Subtraction);
}

pub trait ExpressionBase {
    fn accept(&self, visitor: &dyn ExpressionVisitor);
    fn get_value(&self) -> f32;
}

pub struct Literal {
    value: f32,
}

impl Literal {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl ExpressionBase for Literal {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.visit_literal_expression(self)
    }

    fn get_value(&self) -> f32 {
        self.value
    }
}

pub struct Addition {
    pub left: Box<dyn ExpressionBase>,
    pub right: Box<dyn ExpressionBase>,
}

impl Addition {
    pub fn new(left: Box<dyn ExpressionBase>, right: Box<dyn ExpressionBase>) -> Self {
        Self {
            left,
            right,
        }
    }
}

impl ExpressionBase for Addition {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.visit_addition_expression(self)
    }

    fn get_value(&self) -> f32 {
        self.left.get_value() + self.right.get_value()
    }
}

pub struct Subtraction {
    pub left: Box<dyn ExpressionBase>,
    pub right: Box<dyn ExpressionBase>,
}

impl Subtraction {
    pub fn new(left: Box<dyn ExpressionBase>, right: Box<dyn ExpressionBase>) -> Self {
        Self {
            left,
            right,
        }
    }
}

impl ExpressionBase for Subtraction {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.visit_subtraction_expression(self)
    }

    fn get_value(&self) -> f32 {
        self.left.get_value() - self.right.get_value()
    }
}

pub struct ExpressionPrintingVisitor;

impl ExpressionVisitor for ExpressionPrintingVisitor {
    fn visit_literal_expression(&self, literal: &Literal) {
        println!("{}", literal.get_value());
    }

    fn visit_addition_expression(&self, addition: &Addition) {
        let left = addition.left.get_value();
        let right = addition.right.get_value();
        let sum = addition.get_value();
        println!("{} + {} = {}", left, right, sum);
    }

    fn visit_subtraction_expression(&self, subtraction: &Subtraction) {
        let left = subtraction.left.get_value();
        let right = subtraction.right.get_value();
        let difference = subtraction.get_value();
        println!("{} - {} = {}", left, right, difference);
    }
}

fn main() {
    let visitor = ExpressionPrintingVisitor;

    // Emulate (1 + 2) + 3
    let expr: Box<dyn ExpressionBase> = Box::new(Addition::new(
        Box::new(Addition::new(
            Box::new(Literal::new(1.0)),
            Box::new(Literal::new(2.0)),
        )),
        Box::new(Literal::new(3.0)),
    ));

    expr.accept(&visitor);

    // Emulate 1 - 2 = -1
    let expr: Box<dyn ExpressionBase> = Box::new(Subtraction::new(
        Box::new(Literal::new(1.0)),
        Box::new(Literal::new(2.0))
    ));

    expr.accept(&visitor);

    // Emulate (1 - 2) + 8 = 7
    let expr: Box<dyn ExpressionBase> = Box::new(Addition::new(
        Box::new(Subtraction::new(
            Box::new(Literal::new(1.0)),
            Box::new(Literal::new(2.0))
        )),
        Box::new(Literal::new(8.0))
    ));

    expr.accept(&visitor);
}
