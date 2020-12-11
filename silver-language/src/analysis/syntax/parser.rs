use std::collections::VecDeque;

use super::{
    expression_syntax::ExpressionSyntax, lexer::Lexer, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken, syntax_tree::SyntaxTree,
};

pub(crate) struct Parser<'source> {
    tokens: VecDeque<SyntaxToken<'source>>,
}

impl<'source> Parser<'source> {
    fn new(tokens: VecDeque<SyntaxToken<'source>>) -> Self {
        Self { tokens }
    }

    pub(crate) fn parse(text: &'source str) -> SyntaxTree<'source> {
        let tokens = Lexer::get_tokens(text)
            .iter()
            .filter(|t| t.kind() != SyntaxKind::WhitespaceToken && t.kind() != SyntaxKind::BadToken)
            .cloned()
            .collect();
        let mut parser = Self::new(tokens);
        let expression = parser.parse_expression();
        let end_of_file = parser.match_token(SyntaxKind::EndOfFileToken);
        SyntaxTree::new(expression, end_of_file)
    }

    fn parse_expression(&mut self) -> ExpressionSyntax<'source> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ExpressionSyntax<'source> {
        let mut left = self.parse_factor();

        while self.check_token(&[SyntaxKind::PlusToken, SyntaxKind::MinusToken]) {
            let operator = self.next_token();
            let right = self.parse_factor();

            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> ExpressionSyntax<'source> {
        let mut left = self.parse_primary_expression();

        while self.check_token(&[SyntaxKind::StarToken, SyntaxKind::SlashToken]) {
            let operator = self.next_token();
            let right = self.parse_primary_expression();

            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax<'source> {
        let literal_token = self.match_token(SyntaxKind::NumberToken);
        ExpressionSyntax::Literal { literal_token }
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
        if self.tokens[0].kind() == kind {
            self.next_token()
        } else {
            SyntaxToken::new(kind, self.tokens[0].position(), "", None)
        }
    }
}
