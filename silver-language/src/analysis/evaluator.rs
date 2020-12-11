use super::{
    binding::{
        bound_binary_operator::BoundBinaryOperator,
        bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
        bound_unary_operator::BoundUnaryOperator,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
    },
    silver_value::SilverValue,
};

pub struct Evaluator {
    bound_tree: BoundExpression,
}

impl Evaluator {
    pub(crate) fn new(bound_tree: BoundExpression) -> Self {
        Self { bound_tree }
    }

    pub fn evaluate(&self) -> SilverValue {
        self.evaluate_expression(&self.bound_tree)
    }

    fn evaluate_expression(&self, root: &BoundExpression) -> SilverValue {
        match root {
            BoundExpression::Literal { value } => value.clone().unwrap(),
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
    ) -> SilverValue {
        let left = self.evaluate_expression(left);
        let right = self.evaluate_expression(right);

        match operator.kind() {
            BoundBinaryOperatorKind::Addition => {
                SilverValue::Integer(left.as_integer().unwrap() + right.as_integer().unwrap())
            }
            BoundBinaryOperatorKind::Subtraction => {
                SilverValue::Integer(left.as_integer().unwrap() + right.as_integer().unwrap())
            }
            BoundBinaryOperatorKind::Multiplication => {
                SilverValue::Integer(left.as_integer().unwrap() * right.as_integer().unwrap())
            }
            BoundBinaryOperatorKind::Division => {
                SilverValue::Integer(left.as_integer().unwrap() / right.as_integer().unwrap())
            }
            BoundBinaryOperatorKind::LogicalAnd => {
                SilverValue::Boolean(left.as_boolean().unwrap() && right.as_boolean().unwrap())
            }
            BoundBinaryOperatorKind::LogicalOr => {
                SilverValue::Boolean(left.as_boolean().unwrap() || right.as_boolean().unwrap())
            }
            BoundBinaryOperatorKind::Equality => SilverValue::Boolean(left == right),
            BoundBinaryOperatorKind::Inequality => SilverValue::Boolean(left != right),
        }
    }

    fn evaluate_unary_expression(
        &self,
        operator: &BoundUnaryOperator,
        operand: &BoundExpression,
    ) -> SilverValue {
        let operand = self.evaluate_expression(operand);
        match operator.kind() {
            BoundUnaryOperatorKind::Identity => operand,
            BoundUnaryOperatorKind::Negation => {
                SilverValue::Integer(-operand.as_integer().unwrap())
            }
            BoundUnaryOperatorKind::LogicalNegation => {
                SilverValue::Boolean(!operand.as_boolean().unwrap())
            }
        }
    }
}
