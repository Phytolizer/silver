use crate::analysis::silver_type::SilverType;
use crate::analysis::syntax::syntax_kind::SyntaxKind;

#[derive(Debug)]
pub enum DiagnosticKind {
    BadCharacter,
    BadLiteral(SilverType),
    UnexpectedToken {
        expected_kind: SyntaxKind,
        actual_kind: SyntaxKind,
    },
    UndefinedBinaryOperator {
        operator_kind: SyntaxKind,
        left_type: SilverType,
        right_type: SilverType,
    },
    UndefinedUnaryOperator {
        operator_kind: SyntaxKind,
        operand_type: SilverType,
    },
}
