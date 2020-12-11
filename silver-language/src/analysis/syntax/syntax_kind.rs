use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    // Special tokens
    BadToken,
    EndOfFileToken,

    // Dynamic tokens
    NumberToken,
    WhitespaceToken,

    // Fixed tokens
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    OpenParenthesisToken,
    CloseParenthesisToken,

    // Nodes
    Root,

    // Expressions
    LiteralExpression,
    UnaryExpression,
    BinaryExpression,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
