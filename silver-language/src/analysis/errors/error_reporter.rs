use crate::analysis::diagnostic_kind::DiagnosticKind;
use crate::analysis::syntax::syntax_token::SyntaxToken;
use crate::analysis::{
    diagnostic::Diagnostic, silver_type::SilverType, syntax::syntax_kind::SyntaxKind,
    text::text_span::TextSpan,
};

pub trait ErrorReporter {
    fn report_error(&mut self, error: Diagnostic);
    fn report_invalid_character(&mut self, span: TextSpan, character: char) {
        let message = format!("There is a bad character in the input: '{}'.", character);
        self.report_error(Diagnostic::new(span, message, DiagnosticKind::BadCharacter));
    }
    fn report_invalid_number(&mut self, span: TextSpan, literal: &str, ty: SilverType) {
        let message = format!(
            "The numeric literal '{}' is invalid for the internal integer type '{}'.",
            literal, ty
        );
        self.report_error(Diagnostic::new(
            span,
            message,
            DiagnosticKind::BadLiteral(ty),
        ));
    }
    fn report_unexpected_token(
        &mut self,
        span: TextSpan,
        actual_kind: SyntaxKind,
        expected_kind: SyntaxKind,
    ) {
        let message = format!(
            "Hit an unexpected token <{}>; expected <{}>.",
            actual_kind, expected_kind
        );
        self.report_error(Diagnostic::new(
            span,
            message,
            DiagnosticKind::UnexpectedToken {
                actual_kind,
                expected_kind,
            },
        ));
    }
    fn report_undefined_binary_operator(
        &mut self,
        span: TextSpan,
        operator: SyntaxToken,
        left_type: SilverType,
        right_type: SilverType,
    ) {
        let message = format!(
            "The binary operator '{}' is not defined for types '{}' and '{}'.",
            operator.text(),
            left_type,
            right_type
        );
        self.report_error(Diagnostic::new(
            span,
            message,
            DiagnosticKind::UndefinedBinaryOperator {
                operator_kind: operator.kind(),
                left_type,
                right_type,
            },
        ));
    }
    fn report_undefined_unary_operator(
        &mut self,
        span: TextSpan,
        operator: SyntaxToken,
        operand_type: SilverType,
    ) {
        let message = format!(
            "The unary operator '{}' is not defined for type '{}'.",
            operator.text(),
            operand_type
        );
        self.report_error(Diagnostic::new(
            span,
            message,
            DiagnosticKind::UndefinedUnaryOperator {
                operator_kind: operator.kind(),
                operand_type,
            },
        ));
    }
    fn had_error(&self) -> bool;
    fn errors(&self) -> &[Diagnostic];
    fn clear(&mut self);
}
