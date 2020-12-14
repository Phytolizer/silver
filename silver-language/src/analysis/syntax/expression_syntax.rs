use crate::analysis::{silver_value::SilverValue, text::text_span::TextSpan};

use super::{syntax_kind::SyntaxKind, syntax_node::SyntaxNodeExt, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionSyntax {
    Literal {
        literal_token: SyntaxToken,
        value: Option<SilverValue>,
    },
    Binary {
        left: Box<ExpressionSyntax>,
        operator: SyntaxToken,
        right: Box<ExpressionSyntax>,
    },
    Unary {
        operator: SyntaxToken,
        operand: Box<ExpressionSyntax>,
    },
    Parenthesized {
        open_parenthesis_token: SyntaxToken,
        expression: Box<ExpressionSyntax>,
        close_parenthesis_token: SyntaxToken,
    },
    Name {
        identifier_token: SyntaxToken,
    },
    Assignment {
        identifier_token: SyntaxToken,
        equals_token: SyntaxToken,
        expression: Box<ExpressionSyntax>,
    },
}

impl SyntaxNodeExt for ExpressionSyntax {
    fn kind(&self) -> SyntaxKind {
        match self {
            ExpressionSyntax::Literal { .. } => SyntaxKind::LiteralExpression,
            ExpressionSyntax::Binary { .. } => SyntaxKind::BinaryExpression,
            ExpressionSyntax::Unary { .. } => SyntaxKind::UnaryExpression,
            ExpressionSyntax::Parenthesized { .. } => SyntaxKind::ParenthesizedExpression,
            ExpressionSyntax::Name { .. } => SyntaxKind::NameExpression,
            ExpressionSyntax::Assignment { .. } => SyntaxKind::AssignmentExpression,
        }
    }
    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        match self {
            ExpressionSyntax::Literal { literal_token, .. } => vec![literal_token],
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
            ExpressionSyntax::Name { identifier_token } => vec![identifier_token],
            ExpressionSyntax::Assignment {
                identifier_token,
                equals_token,
                expression,
            } => vec![identifier_token, equals_token, expression.as_ref()],
        }
    }

    fn value(&self) -> Option<&crate::analysis::silver_value::SilverValue> {
        None
    }

    fn text(&self) -> Option<&str> {
        None
    }

    fn span(&self) -> TextSpan {
        match self {
            ExpressionSyntax::Literal { literal_token, .. } => literal_token.span(),
            ExpressionSyntax::Binary { left, right, .. } => left.span().start..right.span().end,
            ExpressionSyntax::Unary { operator, operand } => {
                operator.span().start..operand.span().end
            }
            ExpressionSyntax::Parenthesized {
                open_parenthesis_token,
                close_parenthesis_token,
                ..
            } => open_parenthesis_token.span().start..close_parenthesis_token.span().end,
            ExpressionSyntax::Name { identifier_token } => identifier_token.span(),
            ExpressionSyntax::Assignment {
                identifier_token,
                expression,
                ..
            } => identifier_token.span().start..expression.span().end,
        }
    }
}
