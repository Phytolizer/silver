use derive_more::Constructor;

use crate::analysis::{silver_type::SilverType, syntax::syntax_kind::SyntaxKind};

use super::{
    bound_node::BoundNode, bound_node_kind::BoundNodeKind,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

#[derive(Debug, Clone, Constructor)]
pub(crate) struct BoundUnaryOperator {
    syntax_kind: SyntaxKind,
    kind: BoundUnaryOperatorKind,
    operand_type: SilverType,
    result_type: SilverType,
}

impl BoundUnaryOperator {
    pub(crate) fn kind(&self) -> BoundUnaryOperatorKind {
        self.kind
    }

    pub(crate) fn result_type(&self) -> SilverType {
        self.result_type
    }

    fn operators() -> Vec<Self> {
        vec![
            BoundUnaryOperator::new(
                SyntaxKind::BangToken,
                BoundUnaryOperatorKind::LogicalNegation,
                SilverType::Boolean,
                SilverType::Boolean,
            ),
            BoundUnaryOperator::new(
                SyntaxKind::PlusToken,
                BoundUnaryOperatorKind::Identity,
                SilverType::Integer,
                SilverType::Integer,
            ),
            BoundUnaryOperator::new(
                SyntaxKind::MinusToken,
                BoundUnaryOperatorKind::Negation,
                SilverType::Integer,
                SilverType::Integer,
            ),
        ]
    }

    pub(crate) fn bind(syntax_kind: SyntaxKind, operand_type: SilverType) -> Option<Self> {
        Self::operators()
            .iter()
            .find(|op| op.syntax_kind == syntax_kind && op.operand_type == operand_type)
            .cloned()
    }
}

impl BoundNode for BoundUnaryOperator {
    fn kind(&self) -> super::bound_node_kind::BoundNodeKind {
        BoundNodeKind::UnaryOperator
    }

    fn children(&self) -> Vec<&dyn BoundNode> {
        vec![]
    }
}
