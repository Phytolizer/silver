use std::collections::HashMap;

use super::{
    binding::{
        bound_binary_operator::BoundBinaryOperator,
        bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
        bound_unary_operator::BoundUnaryOperator,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
    },
    silver_value::SilverValue,
    variable_symbol::VariableSymbol,
};

pub struct Evaluator<'variables> {
    variables: &'variables mut HashMap<VariableSymbol, SilverValue>,
}

impl<'variables> Evaluator<'variables> {
    pub(crate) fn new(variables: &'variables mut HashMap<VariableSymbol, SilverValue>) -> Self {
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
            BoundExpression::Variable { variable } => self.evaluate_variable_expression(variable),
            BoundExpression::Assignment {
                variable,
                expression,
            } => self.evaluate_assignment_expression(variable, expression),
        }
    }

    fn evaluate_variable_expression(&self, variable: &VariableSymbol) -> SilverValue {
        self.variables[variable].clone()
    }

    fn evaluate_assignment_expression(
        &mut self,
        variable: &VariableSymbol,
        expression: &BoundExpression,
    ) -> SilverValue {
        let value = self.evaluate_expression(expression);
        self.variables.insert(variable.clone(), value.clone());
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
                SilverValue::Integer(left.as_integer().unwrap() - right.as_integer().unwrap())
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

#[cfg(test)]
mod tests {
    use crate::analysis::{
        compilation::Compilation,
        errors::{error_reporter::ErrorReporter, string_error_reporter::StringErrorReporter},
        syntax::syntax_tree::SyntaxTree,
    };

    use super::*;

    fn check(text: &str, value: &SilverValue) {
        let mut error_reporter = StringErrorReporter::new();
        let syntax_tree = SyntaxTree::parse_str(text, &mut error_reporter);
        let mut compilation = Compilation::new(syntax_tree, &mut error_reporter);
        let mut variables = HashMap::<VariableSymbol, SilverValue>::new();
        let result = compilation.evaluate(&mut variables);
        assert_eq!(value, &result.unwrap());
        assert!(!error_reporter.had_error());
    }

    #[test]
    fn a() {
        for (text, value) in [
            ("1", SilverValue::Integer(1)),
            ("+1", SilverValue::Integer(1)),
            ("-1", SilverValue::Integer(-1)),
            ("1 + 2", SilverValue::Integer(3)),
            ("1 - 2", SilverValue::Integer(-1)),
            ("1 * 2", SilverValue::Integer(2)),
            ("1 / 2", SilverValue::Integer(0)),
            ("(10)", SilverValue::Integer(10)),
            ("12 == 3", SilverValue::Boolean(false)),
            ("3 == 3", SilverValue::Boolean(true)),
            ("12 != 3", SilverValue::Boolean(true)),
            ("3 != 3", SilverValue::Boolean(false)),
            ("true != false", SilverValue::Boolean(true)),
            ("true == false", SilverValue::Boolean(false)),
            ("true", SilverValue::Boolean(true)),
            ("false", SilverValue::Boolean(false)),
            ("!true", SilverValue::Boolean(false)),
            ("!false", SilverValue::Boolean(true)),
            ("a = 10", SilverValue::Integer(10)),
            ("a = true", SilverValue::Boolean(true)),
            ("true && false", SilverValue::Boolean(false)),
            ("true && true", SilverValue::Boolean(true)),
            ("false || false", SilverValue::Boolean(false)),
            ("false || true", SilverValue::Boolean(true)),
        ]
        .iter()
        {
            check(text, value);
        }
    }
}
