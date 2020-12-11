use std::fmt::Display;

use super::silver_type::SilverType;

#[derive(Debug, Clone, PartialEq)]
pub enum SilverValue {
    Integer(i128),
}

impl SilverValue {
    pub fn as_integer(&self) -> Option<i128> {
        match self {
            SilverValue::Integer(i) => Some(*i),
        }
    }

    pub fn ty(&self) -> SilverType {
        match self {
            SilverValue::Integer(_) => SilverType::Integer,
        }
    }
}

impl Display for SilverValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SilverValue::Integer(i) => write!(f, "{}", i),
        }
    }
}
