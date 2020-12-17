use std::sync::Arc;

use parking_lot::RwLock;

use crate::analysis::{
    errors::error_reporter::ErrorReporter,
    silver_value::SilverValue,
    syntax::{
        compilation_unit_syntax::CompilationUnitSyntax, expression_syntax::ExpressionSyntax,
        syntax_token::SyntaxToken,
    },
    variable_symbol::VariableSymbol,
};

use super::{
    bound_binary_operator::BoundBinaryOperator, bound_expression::BoundExpression,
    bound_global_scope::BoundGlobalScope, bound_scope::BoundScope,
    bound_unary_operator::BoundUnaryOperator,
};

pub(crate) struct Binder<'reporter> {
    error_reporter: &'reporter mut dyn ErrorReporter,
    scope: Arc<RwLock<BoundScope>>,
}

impl<'reporter> Binder<'reporter> {
    pub(crate) fn new(
        parent: Option<Arc<RwLock<BoundScope>>>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self {
            error_reporter,
            scope: Arc::new(RwLock::new(BoundScope::new(parent))),
        }
    }

    pub(crate) fn bind(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        self.bind_expression(syntax)
    }

    pub(crate) fn bind_global_scope(
        syntax: &CompilationUnitSyntax,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> BoundGlobalScope {
        let mut binder = Binder::new(None, error_reporter);
        let expression = binder.bind_expression(syntax.expression());
        let variables = binder.scope.read().declared_variables().collect::<Vec<_>>();
        BoundGlobalScope::new(None, variables, expression)
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
        let variable = self.scope.read().try_lookup(name);
        if let Some(variable) = variable {
            BoundExpression::Variable {
                variable: variable.clone(),
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

        let variable = VariableSymbol::new(name.to_string(), bound_expression.ty());
        if !self.scope.write().try_declare(variable.clone()) {
            self.error_reporter
                .report_variable_already_declared(identifier_token.span(), name);
        }
        BoundExpression::Assignment {
            variable,
            expression: Box::new(bound_expression),
        }
    }
}
