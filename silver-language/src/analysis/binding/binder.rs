use crate::analysis::{
    errors::error_reporter::ErrorReporter,
    silver_type::SilverType,
    syntax::{
        expression_syntax::ExpressionSyntax, syntax_kind::SyntaxKind, syntax_token::SyntaxToken,
    },
};

use super::{
    bound_binary_operator::BoundBinaryOperator,
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_unary_operator::BoundUnaryOperator, bound_unary_operator_kind::BoundUnaryOperatorKind,
};

pub(crate) struct Binder<'reporter> {
    error_reporter: &'reporter mut dyn ErrorReporter,
}

impl<'reporter> Binder<'reporter> {
    pub(crate) fn new(error_reporter: &'reporter mut dyn ErrorReporter) -> Self {
        Self { error_reporter }
    }

    pub(crate) fn bind(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        self.bind_expression(syntax)
    }

    fn bind_expression(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        match syntax {
            ExpressionSyntax::Literal { literal_token } => {
                self.bind_literal_expression(literal_token)
            }
            ExpressionSyntax::Binary {
                left,
                operator,
                right,
            } => self.bind_binary_expression(left, operator, right),
            ExpressionSyntax::Unary { operator, operand } => {
                self.bind_unary_expression(operator, operand)
            }
            ExpressionSyntax::Parenthesized {
                open_parenthesis_token,
                expression,
                close_parenthesis_token,
            } => self.bind_parenthesized_expression(
                open_parenthesis_token,
                expression,
                close_parenthesis_token,
            ),
        }
    }

    fn bind_literal_expression(&mut self, literal_token: &SyntaxToken) -> BoundExpression {
        BoundExpression::Literal {
            value: literal_token.value().cloned(),
        }
    }

    fn bind_binary_operator(
        &mut self,
        operator: &SyntaxToken,
        left_type: SilverType,
        right_type: SilverType,
    ) -> Option<BoundBinaryOperator> {
        if left_type != SilverType::Integer || right_type != SilverType::Integer {
            self.error_reporter.report_error(format!(
                "Binary operator '{}' is not defined for types '{}' and '{}'",
                operator.text(),
                left_type,
                right_type
            ));
            return None;
        }
        let kind = match operator.kind() {
            SyntaxKind::PlusToken => BoundBinaryOperatorKind::Addition,
            SyntaxKind::MinusToken => BoundBinaryOperatorKind::Subtraction,
            SyntaxKind::StarToken => BoundBinaryOperatorKind::Multiplication,
            SyntaxKind::SlashToken => BoundBinaryOperatorKind::Division,
            _ => panic!("unexpected binary operator {}", operator.kind()),
        };
        Some(BoundBinaryOperator::new(kind))
    }

    fn bind_binary_expression(
        &mut self,
        left: &ExpressionSyntax,
        operator: &SyntaxToken,
        right: &ExpressionSyntax,
    ) -> BoundExpression {
        let left = self.bind_expression(left);
        let right = self.bind_expression(right);
        let operator = self.bind_binary_operator(operator, left.ty(), right.ty());

        operator
            .map(|operator| BoundExpression::Binary {
                left: Box::new(left.clone()),
                operator,
                right: Box::new(right),
            })
            .unwrap_or(left)
    }

    fn bind_unary_operator(
        &mut self,
        operator: &SyntaxToken,
        operand_type: SilverType,
    ) -> Option<BoundUnaryOperator> {
        if operand_type != SilverType::Integer {
            self.error_reporter.report_error(format!(
                "Unary operator '{}' is not defined for type '{}'",
                operator.text(),
                operand_type
            ));
            return None;
        }
        let kind = match operator.kind() {
            SyntaxKind::PlusToken => BoundUnaryOperatorKind::Identity,
            SyntaxKind::MinusToken => BoundUnaryOperatorKind::Negation,
            _ => panic!("unexpected unary operator {}", operator.kind()),
        };
        Some(BoundUnaryOperator::new(kind))
    }

    fn bind_unary_expression(
        &mut self,
        operator: &SyntaxToken,
        operand: &ExpressionSyntax,
    ) -> BoundExpression {
        let operand = self.bind_expression(operand);
        let operator = self.bind_unary_operator(operator, operand.ty());

        operator
            .map(|operator| BoundExpression::Unary {
                operator,
                operand: Box::new(operand.clone()),
            })
            .unwrap_or(operand)
    }

    // TODO may consider using the tokens in the future
    #[allow(unused_variables)]
    fn bind_parenthesized_expression(
        &mut self,
        open_parenthesis_token: &SyntaxToken,
        expression: &ExpressionSyntax,
        close_parenthesis_token: &SyntaxToken,
    ) -> BoundExpression {
        self.bind_expression(expression)
    }
}
