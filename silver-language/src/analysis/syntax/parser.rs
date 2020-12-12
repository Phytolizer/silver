use std::collections::VecDeque;

use crate::analysis::{errors::error_reporter::ErrorReporter, silver_value::SilverValue};

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
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ExpressionSyntax<'source> {
        if self.peek(0).unwrap().kind() == SyntaxKind::IdentifierToken
            && self.peek(1).unwrap().kind() == SyntaxKind::EqualsToken
        {
            let identifier_token = self.next_token();
            let equals_token = self.next_token();
            let right = self.parse_assignment_expression();
            ExpressionSyntax::Assignment {
                identifier_token,
                equals_token,
                expression: Box::new(right),
            }
        } else {
            self.parse_binary_expression(0)
        }
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
        match self.current().kind() {
            SyntaxKind::OpenParenthesisToken => {
                let open_parenthesis_token = self.next_token();
                let expression = self.parse_expression();
                let close_parenthesis_token = self.match_token(SyntaxKind::CloseParenthesisToken);
                ExpressionSyntax::Parenthesized {
                    open_parenthesis_token,
                    expression: Box::new(expression),
                    close_parenthesis_token,
                }
            }
            SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword => {
                let keyword_token = self.next_token();
                let value = keyword_token.kind() == SyntaxKind::TrueKeyword;
                ExpressionSyntax::Literal {
                    literal_token: keyword_token,
                    value: Some(SilverValue::Boolean(value)),
                }
            }
            SyntaxKind::IdentifierToken => {
                let identifier_token = self.next_token();
                ExpressionSyntax::Name { identifier_token }
            }
            _ => {
                let literal_token = self.match_token(SyntaxKind::NumberToken);
                ExpressionSyntax::Literal {
                    literal_token,
                    value: None,
                }
            }
        }
    }

    fn peek(&self, offset: usize) -> Option<&SyntaxToken> {
        self.tokens.get(offset)
    }

    fn current(&self) -> &SyntaxToken {
        self.peek(0).unwrap()
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
            self.error_reporter.report_unexpected_token(
                self.current().span(),
                self.current().kind(),
                kind,
            );
            SyntaxToken::new(kind, self.tokens[0].position(), "", None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{
        errors::string_error_reporter::StringErrorReporter, syntax::syntax_facts::Operator,
        syntax::syntax_facts::SyntaxKindWithText,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    fn check(input: &str, expected_tree: ExpressionSyntax) {
        let mut error_reporter = StringErrorReporter::new();
        let actual_tree = Parser::parse(input, &mut error_reporter);
        for error in error_reporter.errors() {
            println!("{:?}", error.kind());
        }
        assert!(!error_reporter.had_error(),);
        assert_eq!(&expected_tree, actual_tree.root());
    }

    #[test]
    fn parse_single_number() {
        check(
            "123",
            ExpressionSyntax::Literal {
                literal_token: SyntaxToken::new(
                    SyntaxKind::NumberToken,
                    0,
                    "123",
                    Some(SilverValue::Integer(123)),
                ),
                value: None,
            },
        );
    }

    #[test]
    fn parse_boolean_literals() {
        check(
            "true",
            ExpressionSyntax::Literal {
                literal_token: SyntaxToken::new(SyntaxKind::TrueKeyword, 0, "true", None),
                value: Some(SilverValue::Boolean(true)),
            },
        );
        check(
            "false",
            ExpressionSyntax::Literal {
                literal_token: SyntaxToken::new(SyntaxKind::FalseKeyword, 0, "false", None),
                value: Some(SilverValue::Boolean(false)),
            },
        );
    }

    fn get_unary_operators() -> Vec<(SyntaxKind, &'static str)> {
        SyntaxKind::iter()
            .filter(|k| k.unary_operator_precedence() > 0)
            .map(|k| (k, k.get_text().unwrap()))
            .collect()
    }

    fn get_binary_operators() -> Vec<(SyntaxKind, &'static str)> {
        SyntaxKind::iter()
            .filter(|k| k.binary_operator_precedence() > 0)
            .map(|k| (k, k.get_text().unwrap()))
            .collect()
    }

    #[test]
    fn parse_unary_operators() {
        for (unary_kind, unary_op) in get_unary_operators() {
            check(
                &format!("{}1", unary_op),
                ExpressionSyntax::Unary {
                    operator: SyntaxToken::new(unary_kind, 0, unary_op, None),
                    operand: Box::new(ExpressionSyntax::Literal {
                        value: None,
                        literal_token: SyntaxToken::new(
                            SyntaxKind::NumberToken,
                            1,
                            "1",
                            Some(SilverValue::Integer(1)),
                        ),
                    }),
                },
            );
        }
    }

    #[test]
    fn parse_binary_operators() {
        for (binary_kind, binary_op) in get_binary_operators() {
            check(
                &format!("1{}2", binary_op),
                ExpressionSyntax::Binary {
                    left: Box::new(ExpressionSyntax::Literal {
                        literal_token: SyntaxToken::new(
                            SyntaxKind::NumberToken,
                            0,
                            "1",
                            Some(SilverValue::Integer(1)),
                        ),
                        value: None,
                    }),
                    operator: SyntaxToken::new(binary_kind, 1, binary_op, None),
                    right: Box::new(ExpressionSyntax::Literal {
                        literal_token: SyntaxToken::new(
                            SyntaxKind::NumberToken,
                            1 + binary_op.len(),
                            "2",
                            Some(SilverValue::Integer(2)),
                        ),
                        value: None,
                    }),
                },
            )
        }
    }

    #[test]
    fn parse_parenthesized_expression() {
        check(
            "(1)",
            ExpressionSyntax::Parenthesized {
                open_parenthesis_token: SyntaxToken::new(
                    SyntaxKind::OpenParenthesisToken,
                    0,
                    "(",
                    None,
                ),
                expression: Box::new(ExpressionSyntax::Literal {
                    literal_token: SyntaxToken::new(
                        SyntaxKind::NumberToken,
                        1,
                        "1",
                        Some(SilverValue::Integer(1)),
                    ),
                    value: None,
                }),
                close_parenthesis_token: SyntaxToken::new(
                    SyntaxKind::CloseParenthesisToken,
                    2,
                    ")",
                    None,
                ),
            },
        )
    }

    #[test]
    fn parse_name_expression() {
        check(
            "test",
            ExpressionSyntax::Name {
                identifier_token: SyntaxToken::new(SyntaxKind::IdentifierToken, 0, "test", None),
            },
        )
    }

    #[test]
    fn parse_assignment_expression() {
        check(
            "a=2",
            ExpressionSyntax::Assignment {
                identifier_token: SyntaxToken::new(SyntaxKind::IdentifierToken, 0, "a", None),
                equals_token: SyntaxToken::new(SyntaxKind::EqualsToken, 1, "=", None),
                expression: Box::new(ExpressionSyntax::Literal {
                    literal_token: SyntaxToken::new(
                        SyntaxKind::NumberToken,
                        2,
                        "2",
                        Some(SilverValue::Integer(2)),
                    ),
                    value: None,
                }),
            },
        )
    }

    fn check_binary_operators_parsing(
        op1kind: SyntaxKind,
        op1text: &str,
        op2kind: SyntaxKind,
        op2text: &str,
    ) {
        let op1precedence = op1kind.binary_operator_precedence();
        let op2precedence = op2kind.binary_operator_precedence();
        let input = format!("a{}b{}c", op1text, op2text);
        if op1precedence >= op2precedence {
            check(
                &input,
                ExpressionSyntax::Binary {
                    left: Box::new(ExpressionSyntax::Binary {
                        left: Box::new(ExpressionSyntax::Name {
                            identifier_token: SyntaxToken::new(
                                SyntaxKind::IdentifierToken,
                                0,
                                "a",
                                None,
                            ),
                        }),
                        operator: SyntaxToken::new(op1kind, 1, op1text, None),
                        right: Box::new(ExpressionSyntax::Name {
                            identifier_token: SyntaxToken::new(
                                SyntaxKind::IdentifierToken,
                                1 + op1text.len(),
                                "b",
                                None,
                            ),
                        }),
                    }),
                    operator: SyntaxToken::new(op2kind, 2 + op1text.len(), op2text, None),
                    right: Box::new(ExpressionSyntax::Name {
                        identifier_token: SyntaxToken::new(
                            SyntaxKind::IdentifierToken,
                            2 + op1text.len() + op2text.len(),
                            "c",
                            None,
                        ),
                    }),
                },
            )
        } else {
            check(
                &input,
                ExpressionSyntax::Binary {
                    left: Box::new(ExpressionSyntax::Name {
                        identifier_token: SyntaxToken::new(
                            SyntaxKind::IdentifierToken,
                            0,
                            "a",
                            None,
                        ),
                    }),
                    operator: SyntaxToken::new(op1kind, 1, op1text, None),
                    right: Box::new(ExpressionSyntax::Binary {
                        left: Box::new(ExpressionSyntax::Name {
                            identifier_token: SyntaxToken::new(
                                SyntaxKind::IdentifierToken,
                                1 + op1text.len(),
                                "b",
                                None,
                            ),
                        }),
                        operator: SyntaxToken::new(op2kind, 2 + op1text.len(), op2text, None),
                        right: Box::new(ExpressionSyntax::Name {
                            identifier_token: SyntaxToken::new(
                                SyntaxKind::IdentifierToken,
                                2 + op1text.len() + op2text.len(),
                                "c",
                                None,
                            ),
                        }),
                    }),
                },
            );
        }
    }

    #[test]
    fn binary_operators_respect_precedence() {
        for (op1kind, op1text) in get_binary_operators() {
            for (op2kind, op2text) in get_binary_operators() {
                check_binary_operators_parsing(op1kind, op1text, op2kind, op2text);
            }
        }
    }
}
