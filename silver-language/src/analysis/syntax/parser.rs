use std::{collections::VecDeque, sync::Arc};

use crate::analysis::{
    errors::error_reporter::ErrorReporter, silver_value::SilverValue, text::source_text::SourceText,
};

use super::{
    compilation_unit_syntax::CompilationUnitSyntax, expression_syntax::ExpressionSyntax,
    lexer::Lexer, syntax_facts::Operator, syntax_kind::SyntaxKind, syntax_token::SyntaxToken,
};

pub(crate) struct Parser<'reporter> {
    tokens: VecDeque<SyntaxToken>,
    error_reporter: &'reporter mut dyn ErrorReporter,
}

impl<'reporter> Parser<'reporter> {
    fn new(
        tokens: VecDeque<SyntaxToken>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self {
            tokens,
            error_reporter,
        }
    }

    pub(crate) fn parse_compilation_unit(
        text: Arc<SourceText>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> CompilationUnitSyntax {
        let tokens = Lexer::get_tokens(text.clone(), error_reporter)
            .iter()
            .filter(|t| t.kind() != SyntaxKind::WhitespaceToken && t.kind() != SyntaxKind::BadToken)
            .cloned()
            .collect();
        let mut parser = Self::new(tokens, error_reporter);
        let expression = parser.parse_expression();
        let end_of_file = parser.match_token(SyntaxKind::EndOfFileToken);
        CompilationUnitSyntax::new(expression, end_of_file)
    }

    fn parse_expression(&mut self) -> ExpressionSyntax {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ExpressionSyntax {
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

    fn parse_binary_expression(&mut self, parent_precedence: usize) -> ExpressionSyntax {
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

    fn parse_primary_expression(&mut self) -> ExpressionSyntax {
        match self.current().kind() {
            SyntaxKind::OpenParenthesisToken => self.parse_parenthesized_expression(),
            SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword => self.parse_boolean_literal(),
            SyntaxKind::NumberToken => self.parse_number_literal(),
            _ => self.parse_name_expression(),
        }
    }

    fn parse_number_literal(&mut self) -> ExpressionSyntax {
        let literal_token = self.match_token(SyntaxKind::NumberToken);
        ExpressionSyntax::Literal {
            literal_token,
            value: None,
        }
    }

    fn parse_parenthesized_expression(&mut self) -> ExpressionSyntax {
        let open_parenthesis_token = self.match_token(SyntaxKind::OpenParenthesisToken);
        let expression = self.parse_expression();
        let close_parenthesis_token = self.match_token(SyntaxKind::CloseParenthesisToken);
        ExpressionSyntax::Parenthesized {
            open_parenthesis_token,
            expression: Box::new(expression),
            close_parenthesis_token,
        }
    }

    fn parse_boolean_literal(&mut self) -> ExpressionSyntax {
        let is_true = self.current().kind() == SyntaxKind::TrueKeyword;
        let keyword_token = if is_true {
            self.match_token(SyntaxKind::TrueKeyword)
        } else {
            self.match_token(SyntaxKind::FalseKeyword)
        };
        ExpressionSyntax::Literal {
            literal_token: keyword_token,
            value: Some(SilverValue::Boolean(is_true)),
        }
    }

    fn parse_name_expression(&mut self) -> ExpressionSyntax {
        let identifier_token = self.match_token(SyntaxKind::IdentifierToken);
        ExpressionSyntax::Name { identifier_token }
    }

    fn peek(&self, offset: usize) -> Option<&SyntaxToken> {
        self.tokens.get(offset)
    }

    fn current(&self) -> &SyntaxToken {
        self.peek(0).unwrap()
    }

    fn next_token(&mut self) -> SyntaxToken {
        if self.tokens.len() > 1 {
            self.tokens.pop_front().unwrap()
        } else {
            self.current().clone()
        }
    }

    fn match_token(&mut self, kind: SyntaxKind) -> SyntaxToken {
        if self.current().kind() == kind {
            self.next_token()
        } else {
            self.error_reporter.report_unexpected_token(
                self.current().span(),
                self.current().kind(),
                kind,
            );
            SyntaxToken::new(kind, self.current().position(), String::new(), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{
        diagnostic_kind::DiagnosticKind,
        errors::{
            null_error_reporter::NullErrorReporter, string_error_reporter::StringErrorReporter,
        },
        syntax::syntax_facts::Operator,
        syntax::{
            syntax_facts::SyntaxKindWithText,
            syntax_node::{flatten_tree, SyntaxNodeExt},
            syntax_tree::SyntaxTree,
        },
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    fn check(input: &str, expected_tree: ExpressionSyntax) {
        let mut error_reporter = StringErrorReporter::new();
        let actual_tree =
            Parser::parse_compilation_unit(Arc::new(input.to_string().into()), &mut error_reporter);
        for error in error_reporter.errors() {
            println!("{:?}", error.kind());
        }
        assert!(!error_reporter.had_error(),);
        assert_eq!(&expected_tree, actual_tree.expression());
    }

    #[test]
    fn parse_single_number() {
        check(
            "123",
            ExpressionSyntax::Literal {
                literal_token: SyntaxToken::new(
                    SyntaxKind::NumberToken,
                    0,
                    String::from("123"),
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
                literal_token: SyntaxToken::new(
                    SyntaxKind::TrueKeyword,
                    0,
                    String::from("true"),
                    None,
                ),
                value: Some(SilverValue::Boolean(true)),
            },
        );
        check(
            "false",
            ExpressionSyntax::Literal {
                literal_token: SyntaxToken::new(
                    SyntaxKind::FalseKeyword,
                    0,
                    String::from("false"),
                    None,
                ),
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
                    operator: SyntaxToken::new(unary_kind, 0, unary_op.to_string(), None),
                    operand: Box::new(ExpressionSyntax::Literal {
                        value: None,
                        literal_token: SyntaxToken::new(
                            SyntaxKind::NumberToken,
                            1,
                            String::from("1"),
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
                            String::from("1"),
                            Some(SilverValue::Integer(1)),
                        ),
                        value: None,
                    }),
                    operator: SyntaxToken::new(binary_kind, 1, binary_op.to_string(), None),
                    right: Box::new(ExpressionSyntax::Literal {
                        literal_token: SyntaxToken::new(
                            SyntaxKind::NumberToken,
                            1 + binary_op.len(),
                            String::from("2"),
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
                    String::from("("),
                    None,
                ),
                expression: Box::new(ExpressionSyntax::Literal {
                    literal_token: SyntaxToken::new(
                        SyntaxKind::NumberToken,
                        1,
                        String::from("1"),
                        Some(SilverValue::Integer(1)),
                    ),
                    value: None,
                }),
                close_parenthesis_token: SyntaxToken::new(
                    SyntaxKind::CloseParenthesisToken,
                    2,
                    String::from(")"),
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
                identifier_token: SyntaxToken::new(
                    SyntaxKind::IdentifierToken,
                    0,
                    String::from("test"),
                    None,
                ),
            },
        )
    }

    #[test]
    fn parse_assignment_expression() {
        check(
            "a=2",
            ExpressionSyntax::Assignment {
                identifier_token: SyntaxToken::new(
                    SyntaxKind::IdentifierToken,
                    0,
                    String::from("a"),
                    None,
                ),
                equals_token: SyntaxToken::new(SyntaxKind::EqualsToken, 1, String::from("="), None),
                expression: Box::new(ExpressionSyntax::Literal {
                    literal_token: SyntaxToken::new(
                        SyntaxKind::NumberToken,
                        2,
                        String::from("2"),
                        Some(SilverValue::Integer(2)),
                    ),
                    value: None,
                }),
            },
        )
    }

    struct AssertingIterator<'n> {
        nodes: Vec<&'n dyn SyntaxNodeExt>,
        cursor: usize,
    }

    impl<'n> AssertingIterator<'n> {
        fn new(node: &'n dyn SyntaxNodeExt) -> Self {
            Self {
                nodes: flatten_tree(node),
                cursor: 0,
            }
        }

        fn assert_token(&mut self, kind: SyntaxKind, text: &str) {
            let node = self.nodes[self.cursor];
            self.cursor += 1;

            assert_eq!(kind, node.kind());
            assert_eq!(text, node.text().unwrap());
        }

        fn assert_node(&mut self, kind: SyntaxKind) {
            let node = self.nodes[self.cursor];
            self.cursor += 1;

            assert_eq!(kind, node.kind());
            assert!(node.text().is_none());
        }

        fn assert_at_end(&self) {
            dbg!(self.nodes.iter().map(|n| n.kind()).collect::<Vec<_>>());
            assert_eq!(self.nodes.len(), self.cursor);
        }
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
        let tree = SyntaxTree::parse_str(&input, &mut NullErrorReporter::new());

        let mut e = AssertingIterator::new(tree.root());

        if op1precedence >= op2precedence {
            e.assert_node(SyntaxKind::CompilationUnit);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "a");
            e.assert_token(op1kind, op1text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "b");
            e.assert_token(op2kind, op2text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "c");
            e.assert_token(SyntaxKind::EndOfFileToken, "");
        } else {
            e.assert_node(SyntaxKind::CompilationUnit);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "a");
            e.assert_token(op1kind, op1text);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "b");
            e.assert_token(op2kind, op2text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "c");
            e.assert_token(SyntaxKind::EndOfFileToken, "");
        }
        e.assert_at_end();
    }

    #[test]
    fn binary_operators_respect_precedence() {
        for (op1kind, op1text) in get_binary_operators() {
            for (op2kind, op2text) in get_binary_operators() {
                check_binary_operators_parsing(op1kind, op1text, op2kind, op2text);
            }
        }
    }

    fn check_unary_binary_operators_parsing(
        op1kind: SyntaxKind,
        op1text: &str,
        op2kind: SyntaxKind,
        op2text: &str,
    ) {
        let op1precedence = op1kind.unary_operator_precedence();
        let op2precedence = op2kind.binary_operator_precedence();
        let input = format!("{}a{}b", op1text, op2text);
        let tree = SyntaxTree::parse_str(&input, &mut NullErrorReporter::new());

        let mut e = AssertingIterator::new(tree.root());

        if op1precedence >= op2precedence {
            e.assert_node(SyntaxKind::CompilationUnit);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::UnaryExpression);
            e.assert_token(op1kind, op1text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "a");
            e.assert_token(op2kind, op2text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "b");
            e.assert_token(SyntaxKind::EndOfFileToken, "");
        } else {
            e.assert_node(SyntaxKind::CompilationUnit);
            e.assert_node(SyntaxKind::UnaryExpression);
            e.assert_token(op1kind, op1text);
            e.assert_node(SyntaxKind::BinaryExpression);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "a");
            e.assert_token(op2kind, op2text);
            e.assert_node(SyntaxKind::NameExpression);
            e.assert_token(SyntaxKind::IdentifierToken, "b");
            e.assert_token(SyntaxKind::EndOfFileToken, "");
        }
        e.assert_at_end();
    }

    #[test]
    fn unary_and_binary_operators_respect_precedence() {
        for (op1kind, op1text) in get_unary_operators() {
            for (op2kind, op2text) in get_binary_operators() {
                check_unary_binary_operators_parsing(op1kind, op1text, op2kind, op2text);
            }
        }
    }

    fn check_bad(input: &str, expected_errors: Vec<DiagnosticKind>) {
        let mut error_reporter = StringErrorReporter::new();
        Parser::parse_compilation_unit(
            Arc::new(SourceText::from(input.to_string())),
            &mut error_reporter,
        );
        assert_eq!(expected_errors.len(), error_reporter.errors().len());
        for (expected_error, actual_error) in
            expected_errors.iter().zip(error_reporter.errors().iter())
        {
            assert_eq!(expected_error, actual_error.kind());
        }
    }

    #[test]
    fn empty_input() {
        check_bad(
            "",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::IdentifierToken,
                actual_kind: SyntaxKind::EndOfFileToken,
            }],
        )
    }

    #[test]
    fn missing_unary_operand() {
        check_bad(
            "+",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::IdentifierToken,
                actual_kind: SyntaxKind::EndOfFileToken,
            }],
        );
    }

    #[test]
    fn missing_binary_operand_left() {
        check_bad(
            "*1",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::IdentifierToken,
                actual_kind: SyntaxKind::StarToken,
            }],
        );
    }

    #[test]
    fn missing_binary_operand_right() {
        check_bad(
            "1+",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::IdentifierToken,
                actual_kind: SyntaxKind::EndOfFileToken,
            }],
        );
    }

    #[test]
    fn missing_open_parenthesis() {
        check_bad(
            "1)",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::EndOfFileToken,
                actual_kind: SyntaxKind::CloseParenthesisToken,
            }],
        );
    }

    #[test]
    fn missing_close_parenthesis() {
        check_bad(
            "(1",
            vec![DiagnosticKind::UnexpectedToken {
                expected_kind: SyntaxKind::CloseParenthesisToken,
                actual_kind: SyntaxKind::EndOfFileToken,
            }],
        );
    }
}
