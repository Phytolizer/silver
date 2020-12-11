use derive_more::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub enum SilverType {
    Null,
    Integer,
}
