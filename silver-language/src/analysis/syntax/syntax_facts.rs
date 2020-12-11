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

pub(crate) fn keyword_kind(text: &str) -> SyntaxKind {
    match text {
        "true" => SyntaxKind::TrueKeyword,
        "false" => SyntaxKind::FalseKeyword,
        _ => SyntaxKind::IdentifierToken,
    }
}
