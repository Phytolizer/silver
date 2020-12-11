use crate::analysis::{silver_type::SilverType, silver_value::SilverValue};

use super::{
    bound_binary_operator::BoundBinaryOperator, bound_node::BoundNode,
    bound_node_kind::BoundNodeKind, bound_unary_operator::BoundUnaryOperator,
};

#[derive(Debug, Clone)]
pub(crate) enum BoundExpression {
    Literal {
        value: Option<SilverValue>,
    },
    Unary {
        operator: BoundUnaryOperator,
        operand: Box<BoundExpression>,
    },
    Binary {
        left: Box<BoundExpression>,
        operator: BoundBinaryOperator,
        right: Box<BoundExpression>,
    },
}

impl BoundExpression {
    pub(crate) fn ty(&self) -> SilverType {
        match self {
            BoundExpression::Literal { value } => {
                value.as_ref().map(|v| v.ty()).unwrap_or(SilverType::Null)
            }
            BoundExpression::Unary { operand, .. } => operand.ty(),
            BoundExpression::Binary { left, .. } => left.ty(),
        }
    }
}

impl BoundNode for BoundExpression {
    fn kind(&self) -> super::bound_node_kind::BoundNodeKind {
        match self {
            BoundExpression::Literal { .. } => BoundNodeKind::LiteralExpression,
            BoundExpression::Unary { .. } => BoundNodeKind::UnaryExpression,
            BoundExpression::Binary { .. } => BoundNodeKind::BinaryExpression,
        }
    }

    fn children(&self) -> Vec<&dyn BoundNode> {
        match self {
            BoundExpression::Literal { .. } => vec![],
            BoundExpression::Unary { operator, operand } => vec![operator, operand.as_ref()],
            BoundExpression::Binary {
                left,
                operator,
                right,
            } => vec![left.as_ref(), operator, right.as_ref()],
        }
    }
}
