use super::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_node::BoundNode,
    bound_node_kind::BoundNodeKind,
};
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub(crate) struct BoundBinaryOperator {
    kind: BoundBinaryOperatorKind,
}

impl BoundBinaryOperator {
    pub(crate) fn kind(&self) -> BoundBinaryOperatorKind {
        self.kind
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
