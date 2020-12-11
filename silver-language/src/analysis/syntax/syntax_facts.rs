use super::syntax_kind::SyntaxKind;

pub trait Operator {
    fn unary_operator_precedence(&self) -> usize;
    fn binary_operator_precedence(&self) -> usize;
}

impl Operator for SyntaxKind {
    fn unary_operator_precedence(&self) -> usize {
        todo!()
    }

    fn binary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::StarToken | SyntaxKind::SlashToken => 2,
            SyntaxKind::PlusToken | SyntaxKind::MinusToken => 1,
            _ => 0,
        }
    }
}
