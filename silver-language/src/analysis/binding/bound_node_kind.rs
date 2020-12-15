use derive_more::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub(crate) enum BoundNodeKind {
    // Statements
    BlockStatement,
    VariableDeclarationStatement,
    ExpressionStatement,

    // Expressions
    LiteralExpression,
    UnaryExpression,
    BinaryExpression,
    VariableExpression,
    AssignmentExpression,

    // Operators
    UnaryOperator,
    BinaryOperator,
}
