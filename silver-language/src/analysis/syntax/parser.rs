use std::collections::VecDeque;

use crate::analysis::errors::error_reporter::ErrorReporter;

use super::{
    expression_syntax::ExpressionSyntax, lexer::Lexer, syntax_facts::Operator,
    syntax_kind::SyntaxKind, syntax_token::SyntaxToken, syntax_tree::SyntaxTree,
};

pub(crate) struct Parser<'source, 'reporter> {
    tokens: VecDeque<SyntaxToken<'source>>,
    error_reporter: &'reporter mut dyn ErrorReporter,
}

impl<'source, 'reporter> Parser<'source, 'reporter> {
    fn new(
        tokens: VecDeque<SyntaxToken<'source>>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self {
            tokens,
            error_reporter,
        }
    }

    pub(crate) fn parse(
        text: &'source str,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> SyntaxTree<'source> {
        let tokens = Lexer::get_tokens(text, error_reporter)
            .iter()
            .filter(|t| t.kind() != SyntaxKind::WhitespaceToken && t.kind() != SyntaxKind::BadToken)
            .cloned()
            .collect();
        let mut parser = Self::new(tokens, error_reporter);
        let expression = parser.parse_expression();
        let end_of_file = parser.match_token(SyntaxKind::EndOfFileToken);
        SyntaxTree::new(expression, end_of_file)
    }

    fn parse_expression(&mut self) -> ExpressionSyntax<'source> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, parent_precedence: usize) -> ExpressionSyntax<'source> {
        let unary_operator_precedence = self.current().kind().unary_operator_precedence();
        let mut left =
            if unary_operator_precedence != 0 && unary_operator_precedence >= parent_precedence {
                let operator = self.next_token();
                let operand = self.parse_binary_expression(unary_operator_precedence);
                ExpressionSyntax::Unary {
                    operator,
                    operand: Box::new(operand),
                }
            } else {
                self.parse_primary_expression()
            };

        loop {
            let precedence = self.current().kind().binary_operator_precedence();
            if precedence == 0 || precedence <= parent_precedence {
                break;
            }

            let operator = self.next_token();
            let right = self.parse_binary_expression(precedence);
            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax<'source> {
        if self.check_token(&[SyntaxKind::OpenParenthesisToken]) {
            let open_parenthesis_token = self.next_token();
            let expression = self.parse_expression();
            let close_parenthesis_token = self.match_token(SyntaxKind::CloseParenthesisToken);
            return ExpressionSyntax::Parenthesized {
                open_parenthesis_token,
                expression: Box::new(expression),
                close_parenthesis_token,
            };
        }
        let literal_token = self.match_token(SyntaxKind::NumberToken);
        ExpressionSyntax::Literal { literal_token }
    }

    fn current(&self) -> &SyntaxToken {
        &self.tokens[0]
    }

    fn check_token(&self, kinds: &[SyntaxKind]) -> bool {
        for &kind in kinds {
            if self.tokens[0].kind() == kind {
                return true;
            }
        }
        false
    }

    fn next_token(&mut self) -> SyntaxToken<'source> {
        if self.tokens.len() > 1 {
            self.tokens.pop_front().unwrap()
        } else {
            self.tokens[0].clone()
        }
    }

    fn match_token(&mut self, kind: SyntaxKind) -> SyntaxToken<'source> {
        if self.current().kind() == kind {
            self.next_token()
        } else {
            self.error_reporter.report_error(format!(
                "Unexpected token <{}>, expected <{}>",
                self.current().kind(),
                kind
            ));
            SyntaxToken::new(kind, self.tokens[0].position(), "", None)
        }
    }
}
