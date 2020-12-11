use crate::analysis::{
    diagnostic::Diagnostic, silver_type::SilverType, syntax::syntax_kind::SyntaxKind,
    text::text_span::TextSpan,
};

pub trait ErrorReporter {
    fn report_error(&mut self, error: Diagnostic);
    fn report_invalid_character(&mut self, span: TextSpan, character: char) {
        let message = format!("There is a bad character in input: '{}'.", character);
        self.report_error(Diagnostic::new(span, message));
    }
    fn report_invalid_number(&mut self, span: TextSpan, literal: &str, ty: &'static str) {
        let message = format!(
            "The numeric literal '{}' is invalid for internal integer type '{}'.",
            literal, ty
        );
        self.report_error(Diagnostic::new(span, message));
    }
    fn report_unexpected_token(
        &mut self,
        span: TextSpan,
        actual_kind: SyntaxKind,
        expected_kind: SyntaxKind,
    ) {
        let message = format!(
            "Unexpected token <{}>, expected <{}>.",
            actual_kind, expected_kind
        );
        self.report_error(Diagnostic::new(span, message));
    }
    fn report_undefined_binary_operator(
        &mut self,
        span: TextSpan,
        operator: &str,
        left_type: SilverType,
        right_type: SilverType,
    ) {
        let message = format!(
            "Binary operator '{}' is not defined for types '{}' and '{}'.",
            operator, left_type, right_type
        );
        self.report_error(Diagnostic::new(span, message));
    }
    fn report_undefined_unary_operator(
        &mut self,
        span: TextSpan,
        operator: &str,
        operand_type: SilverType,
    ) {
        let message = format!(
            "Unary operator '{}' is not defined for type '{}'.",
            operator, operand_type
        );
        self.report_error(Diagnostic::new(span, message));
    }
    fn had_error(&self) -> bool;
    fn errors(&self) -> &[Diagnostic];
    fn clear(&mut self);
}
