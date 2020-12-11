use crate::analysis::{silver_type::SilverType, syntax::syntax_kind::SyntaxKind};

use super::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_node::BoundNode,
    bound_node_kind::BoundNodeKind,
};
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub(crate) struct BoundBinaryOperator {
    syntax_kind: SyntaxKind,
    kind: BoundBinaryOperatorKind,
    left_type: SilverType,
    right_type: SilverType,
    result_type: SilverType,
}

impl BoundBinaryOperator {
    pub(crate) fn kind(&self) -> BoundBinaryOperatorKind {
        self.kind
    }

    pub(crate) fn result_type(&self) -> SilverType {
        self.result_type
    }

    fn operators() -> Vec<Self> {
        vec![
            BoundBinaryOperator::new(
                SyntaxKind::PlusToken,
                BoundBinaryOperatorKind::Addition,
                SilverType::Integer,
                SilverType::Integer,
                SilverType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::MinusToken,
                BoundBinaryOperatorKind::Subtraction,
                SilverType::Integer,
                SilverType::Integer,
                SilverType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::StarToken,
                BoundBinaryOperatorKind::Multiplication,
                SilverType::Integer,
                SilverType::Integer,
                SilverType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::SlashToken,
                BoundBinaryOperatorKind::Division,
                SilverType::Integer,
                SilverType::Integer,
                SilverType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::AmpersandAmpersandToken,
                BoundBinaryOperatorKind::LogicalAnd,
                SilverType::Boolean,
                SilverType::Boolean,
                SilverType::Boolean,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::PipePipeToken,
                BoundBinaryOperatorKind::LogicalOr,
                SilverType::Boolean,
                SilverType::Boolean,
                SilverType::Boolean,
            ),
        ]
    }

    pub(crate) fn bind(
        syntax_kind: SyntaxKind,
        left_type: SilverType,
        right_type: SilverType,
    ) -> Option<Self> {
        Self::operators()
            .iter()
            .find(|op| {
                op.syntax_kind == syntax_kind
                    && op.left_type == left_type
                    && op.right_type == right_type
            })
            .cloned()
    }
}

impl BoundNode for BoundBinaryOperator {
    fn kind(&self) -> super::bound_node_kind::BoundNodeKind {
        BoundNodeKind::BinaryOperator
    }

    fn children(&self) -> Vec<&dyn BoundNode> {
        vec![]
    }
}
