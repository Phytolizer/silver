use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum SilverValue {
    Integer(i128),
}

impl Display for SilverValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SilverValue::Integer(i) => write!(f, "{}", i),
        }
    }
}
