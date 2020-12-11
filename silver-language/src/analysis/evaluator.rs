use super::{
    binding::{
        binder::Binder, bound_binary_operator::BoundBinaryOperator,
        bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
        bound_unary_operator::BoundUnaryOperator,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
    },
    errors::error_reporter::ErrorReporter,
    silver_value::SilverValue,
    syntax::syntax_tree::SyntaxTree,
};

pub struct Evaluator {
    bound_tree: BoundExpression,
}

impl Evaluator {
    pub fn new(syntax_tree: SyntaxTree, error_reporter: &mut dyn ErrorReporter) -> Self {
        let mut binder = Binder::new(error_reporter);
        let bound_tree = binder.bind(syntax_tree.root());
        Self { bound_tree }
    }

    pub fn evaluate(&self) -> Option<SilverValue> {
        self.evaluate_expression(&self.bound_tree)
    }

    fn evaluate_expression(&self, root: &BoundExpression) -> Option<SilverValue> {
        match root {
            BoundExpression::Literal { value } => value.clone(),
            BoundExpression::Unary { operator, operand } => {
                self.evaluate_unary_expression(operator, operand)
            }
            BoundExpression::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary_expression(left, operator, right),
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: &BoundExpression,
        operator: &BoundBinaryOperator,
        right: &BoundExpression,
    ) -> Option<SilverValue> {
        let left = self.evaluate_expression(left);
        let right = self.evaluate_expression(right);

        match operator.kind() {
            BoundBinaryOperatorKind::Addition => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() + right.unwrap().as_integer().unwrap(),
            )),
            BoundBinaryOperatorKind::Subtraction => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() + right.unwrap().as_integer().unwrap(),
            )),
            BoundBinaryOperatorKind::Multiplication => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() * right.unwrap().as_integer().unwrap(),
            )),
            BoundBinaryOperatorKind::Division => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() / right.unwrap().as_integer().unwrap(),
            )),
        }
    }

    fn evaluate_unary_expression(
        &self,
        operator: &BoundUnaryOperator,
        operand: &BoundExpression,
    ) -> Option<SilverValue> {
        let operand = self.evaluate_expression(operand);
        match operator.kind() {
            BoundUnaryOperatorKind::Identity => operand,
            BoundUnaryOperatorKind::Negation => Some(SilverValue::Integer(
                -operand.unwrap().as_integer().unwrap(),
            )),
        }
    }
}
