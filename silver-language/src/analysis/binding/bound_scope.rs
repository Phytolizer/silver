use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::analysis::variable_symbol::VariableSymbol;

pub(crate) struct BoundScope {
    variables: HashMap<String, VariableSymbol>,
    parent: Option<Arc<RwLock<BoundScope>>>,
}

impl BoundScope {
    pub(crate) fn new(parent: Option<Arc<RwLock<BoundScope>>>) -> Self {
        Self {
            variables: HashMap::new(),
            parent,
        }
    }

    pub(crate) fn try_declare(&mut self, variable: VariableSymbol) -> bool {
        if self.variables.contains_key(variable.name()) {
            return false;
        }

        self.variables.insert(variable.name().to_string(), variable);
        true
    }

    pub(crate) fn try_lookup(&self, name: &str) -> Option<VariableSymbol> {
        self.variables
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.read().try_lookup(name))
    }

    pub(crate) fn declared_variables(&self) -> impl Iterator<Item = VariableSymbol> + '_ {
        self.variables.values().cloned()
    }
}
