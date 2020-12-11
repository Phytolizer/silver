use super::bound_node_kind::BoundNodeKind;

pub(crate) trait BoundNode {
    fn kind(&self) -> BoundNodeKind;
    fn children(&self) -> Vec<&dyn BoundNode>;
}
