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
    Variable {
        name: String,
        ty: SilverType,
    },
    Assignment {
        name: String,
        expression: Box<BoundExpression>,
    },
}

impl BoundExpression {
    pub(crate) fn ty(&self) -> SilverType {
        match self {
            BoundExpression::Literal { value } => {
                value.as_ref().map(|v| v.ty()).unwrap_or(SilverType::Null)
            }
            BoundExpression::Unary { operator, .. } => operator.result_type(),
            BoundExpression::Binary { operator, .. } => operator.result_type(),
            BoundExpression::Variable { ty, .. } => *ty,
            BoundExpression::Assignment { expression, .. } => expression.ty(),
        }
    }
}

impl BoundNode for BoundExpression {
    fn kind(&self) -> super::bound_node_kind::BoundNodeKind {
        match self {
            BoundExpression::Literal { .. } => BoundNodeKind::LiteralExpression,
            BoundExpression::Unary { .. } => BoundNodeKind::UnaryExpression,
            BoundExpression::Binary { .. } => BoundNodeKind::BinaryExpression,
            BoundExpression::Variable { .. } => BoundNodeKind::VariableExpression,
            BoundExpression::Assignment { .. } => BoundNodeKind::AssignmentExpression,
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
            BoundExpression::Variable { .. } => vec![],
            BoundExpression::Assignment { expression, .. } => vec![expression.as_ref()],
        }
    }
}
