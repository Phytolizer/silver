use super::syntax_kind::SyntaxKind;

pub(crate) trait Operator {
    fn unary_operator_precedence(&self) -> usize;
    fn binary_operator_precedence(&self) -> usize;
}

impl Operator for SyntaxKind {
    fn unary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::PlusToken | SyntaxKind::MinusToken | SyntaxKind::BangToken => 6,
            _ => 0,
        }
    }

    fn binary_operator_precedence(&self) -> usize {
        match self {
            // */
            SyntaxKind::StarToken | SyntaxKind::SlashToken => 5,
            // +-
            SyntaxKind::PlusToken | SyntaxKind::MinusToken => 4,

            SyntaxKind::EqualsEqualsToken | SyntaxKind::BangEqualsToken => 3,

            // &&
            SyntaxKind::AmpersandAmpersandToken => 2,

            // ||
            SyntaxKind::PipePipeToken => 1,
            _ => 0,
        }
    }
}

pub(crate) trait SyntaxKindWithText {
    fn get_text(&self) -> Option<&'static str>;
}

impl SyntaxKindWithText for SyntaxKind {
    fn get_text(&self) -> Option<&'static str> {
        match self {
            SyntaxKind::PlusToken => Some("+"),
            SyntaxKind::MinusToken => Some("-"),
            SyntaxKind::StarToken => Some("*"),
            SyntaxKind::SlashToken => Some("/"),
            SyntaxKind::OpenParenthesisToken => Some("("),
            SyntaxKind::CloseParenthesisToken => Some(")"),
            SyntaxKind::BangToken => Some("!"),
            SyntaxKind::AmpersandAmpersandToken => Some("&&"),
            SyntaxKind::PipePipeToken => Some("||"),
            SyntaxKind::EqualsEqualsToken => Some("=="),
            SyntaxKind::BangEqualsToken => Some("!="),
            SyntaxKind::EqualsToken => Some("="),
            SyntaxKind::TrueKeyword => Some("true"),
            SyntaxKind::FalseKeyword => Some("false"),
            _ => None,
        }
    }
}

pub(crate) fn keyword_kind(text: &str) -> SyntaxKind {
    match text {
        "true" => SyntaxKind::TrueKeyword,
        "false" => SyntaxKind::FalseKeyword,
        _ => SyntaxKind::IdentifierToken,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::analysis::{
        errors::null_error_reporter::NullErrorReporter, syntax::syntax_tree::SyntaxTree,
    };

    use super::*;
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    fn get_text_round_trips() {
        for (kind, text) in SyntaxKind::iter().filter_map(|k| k.get_text().map(|t| (k, t))) {
            let tokens = SyntaxTree::parse_tokens(
                Arc::new(text.to_string().into()),
                &mut NullErrorReporter::new(),
            );
            assert_eq!(tokens.len(), 2);
            assert_eq!(tokens[0].kind(), kind);
            assert_eq!(tokens[0].text(), text);
        }
    }
}
