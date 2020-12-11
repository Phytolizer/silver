use derive_more::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub(crate) enum BoundUnaryOperatorKind {
    Identity,
    Negation,
    LogicalNegation,
}
