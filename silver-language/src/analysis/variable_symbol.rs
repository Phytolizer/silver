use super::silver_type::SilverType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VariableSymbol {
    name: String,
    ty: SilverType,
}

impl VariableSymbol {
    pub(crate) fn new(name: String, ty: SilverType) -> Self {
        Self { name, ty }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> SilverType {
        self.ty
    }
}
