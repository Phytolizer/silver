use super::{syntax_kind::SyntaxKind, syntax_node::SyntaxNodeExt, syntax_token::SyntaxToken};

pub enum ExpressionSyntax<'source> {
    Literal {
        literal_token: SyntaxToken<'source>,
    },
    Binary {
        left: Box<ExpressionSyntax<'source>>,
        operator: SyntaxToken<'source>,
        right: Box<ExpressionSyntax<'source>>,
    },
    Unary {
        operator: SyntaxToken<'source>,
        operand: Box<ExpressionSyntax<'source>>,
    },
    Parenthesized {
        open_parenthesis_token: SyntaxToken<'source>,
        expression: Box<ExpressionSyntax<'source>>,
        close_parenthesis_token: SyntaxToken<'source>,
    },
}

impl<'source> SyntaxNodeExt for ExpressionSyntax<'source> {
    fn kind(&self) -> SyntaxKind {
        match self {
            ExpressionSyntax::Literal { .. } => SyntaxKind::LiteralExpression,
            ExpressionSyntax::Binary { .. } => SyntaxKind::BinaryExpression,
            ExpressionSyntax::Unary { .. } => SyntaxKind::UnaryExpression,
            ExpressionSyntax::Parenthesized { .. } => SyntaxKind::ParenthesizedExpression,
        }
    }
    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        match self {
            ExpressionSyntax::Literal { literal_token } => vec![literal_token],
            ExpressionSyntax::Binary {
                left,
                operator,
                right,
            } => vec![left.as_ref(), operator, right.as_ref()],
            ExpressionSyntax::Unary { operator, operand } => vec![operator, operand.as_ref()],
            ExpressionSyntax::Parenthesized {
                open_parenthesis_token,
                expression,
                close_parenthesis_token,
            } => vec![
                open_parenthesis_token,
                expression.as_ref(),
                close_parenthesis_token,
            ],
        }
    }

    fn value(&self) -> Option<&crate::analysis::silver_value::SilverValue> {
        None
    }
}
