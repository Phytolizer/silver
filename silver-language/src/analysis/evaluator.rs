use std::collections::HashMap;

use super::{
    binding::{
        bound_binary_operator::BoundBinaryOperator,
        bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
        bound_unary_operator::BoundUnaryOperator,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
    },
    silver_value::SilverValue,
};

pub struct Evaluator<'variables> {
    variables: &'variables mut HashMap<String, SilverValue>,
}

impl<'variables> Evaluator<'variables> {
    pub(crate) fn new(variables: &'variables mut HashMap<String, SilverValue>) -> Self {
        Self { variables }
    }

    pub(crate) fn evaluate(&mut self, bound_tree: &BoundExpression) -> SilverValue {
        self.evaluate_expression(bound_tree)
    }

    fn evaluate_expression<'a>(&'a mut self, root: &'a BoundExpression) -> SilverValue {
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
            BoundExpression::Variable { name, .. } => self.evaluate_variable_expression(name),
            BoundExpression::Assignment { name, expression } => {
                self.evaluate_assignment_expression(name, expression)
            }
        }
    }

    fn evaluate_variable_expression(&self, name: &str) -> SilverValue {
        self.variables[name].clone()
    }

    fn evaluate_assignment_expression(
        &mut self,
        name: &str,
        expression: &BoundExpression,
    ) -> SilverValue {
        let value = self.evaluate_expression(expression);
        self.variables.insert(name.to_string(), value.clone());
        value
    }

    fn evaluate_binary_expression(
        &mut self,
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
        &mut self,
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
