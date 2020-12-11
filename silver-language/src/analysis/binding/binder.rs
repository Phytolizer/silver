use std::collections::HashMap;

use crate::analysis::{
    errors::error_reporter::ErrorReporter,
    silver_value::SilverValue,
    syntax::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken},
};

use super::{
    bound_binary_operator::BoundBinaryOperator, bound_expression::BoundExpression,
    bound_unary_operator::BoundUnaryOperator,
};

pub(crate) struct Binder<'reporter, 'variables> {
    error_reporter: &'reporter mut dyn ErrorReporter,
    variables: &'variables mut HashMap<String, SilverValue>,
}

impl<'reporter, 'variables> Binder<'reporter, 'variables> {
    pub(crate) fn new(
        variables: &'variables mut HashMap<String, SilverValue>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self {
            error_reporter,
            variables,
        }
    }

    pub(crate) fn bind(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        self.bind_expression(syntax)
    }

    fn bind_expression(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        match syntax {
            ExpressionSyntax::Literal {
                literal_token,
                value,
            } => self.bind_literal_expression(literal_token, value.clone()),
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
            ExpressionSyntax::Name { identifier_token } => {
                self.bind_name_expression(identifier_token)
            }
            ExpressionSyntax::Assignment {
                identifier_token,
                expression,
                ..
            } => self.bind_assignment_expression(identifier_token, expression),
        }
    }

    fn bind_literal_expression(
        &mut self,
        literal_token: &SyntaxToken,
        value: Option<SilverValue>,
    ) -> BoundExpression {
        value
            .map(|v| BoundExpression::Literal { value: Some(v) })
            .unwrap_or(BoundExpression::Literal {
                value: literal_token.value().cloned(),
            })
    }

    fn bind_binary_expression(
        &mut self,
        left: &ExpressionSyntax,
        operator: &SyntaxToken,
        right: &ExpressionSyntax,
    ) -> BoundExpression {
        let left = self.bind_expression(left);
        let right = self.bind_expression(right);
        let bound_operator = BoundBinaryOperator::bind(operator.kind(), left.ty(), right.ty());

        if let Some(operator) = bound_operator {
            BoundExpression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        } else {
            self.error_reporter.report_undefined_binary_operator(
                operator.span(),
                operator.clone(),
                left.ty(),
                right.ty(),
            );
            left
        }
    }

    fn bind_unary_expression(
        &mut self,
        operator: &SyntaxToken,
        operand: &ExpressionSyntax,
    ) -> BoundExpression {
        let operand = self.bind_expression(operand);
        let bound_operator = BoundUnaryOperator::bind(operator.kind(), operand.ty());

        if let Some(operator) = bound_operator {
            BoundExpression::Unary {
                operator,
                operand: Box::new(operand),
            }
        } else {
            self.error_reporter.report_undefined_unary_operator(
                operator.span(),
                operator.clone(),
                operand.ty(),
            );
            operand
        }
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

    fn bind_name_expression(&mut self, identifier_token: &SyntaxToken) -> BoundExpression {
        let name = identifier_token.text();
        if let Some(value) = self.variables.get(name).as_ref() {
            let ty = value.ty();
            BoundExpression::Variable {
                name: name.to_string(),
                ty,
            }
        } else {
            self.error_reporter
                .report_undefined_name(identifier_token.span(), name);
            BoundExpression::Literal {
                value: Some(SilverValue::Integer(0)),
            }
        }
    }

    fn bind_assignment_expression(
        &mut self,
        identifier_token: &SyntaxToken,
        expression: &ExpressionSyntax,
    ) -> BoundExpression {
        let name = identifier_token.text();
        let bound_expression = self.bind_expression(expression);
        BoundExpression::Assignment {
            name: name.to_string(),
            expression: Box::new(bound_expression),
        }
    }
}
