use derive_more::Constructor;

use super::{
    bound_node::BoundNode, bound_node_kind::BoundNodeKind,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

#[derive(Debug, Clone, Constructor)]
pub(crate) struct BoundUnaryOperator {
    kind: BoundUnaryOperatorKind,
}

impl BoundUnaryOperator {
    pub(crate) fn kind(&self) -> BoundUnaryOperatorKind {
        self.kind
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
