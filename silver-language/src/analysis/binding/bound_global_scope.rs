use std::sync::Arc;

use crate::analysis::variable_symbol::VariableSymbol;

use super::bound_expression::BoundExpression;

pub(crate) struct BoundGlobalScope {
    previous: Option<Arc<BoundGlobalScope>>,
    variables: Vec<VariableSymbol>,
    expression: BoundExpression,
}

impl BoundGlobalScope {
    pub(crate) fn new(
        previous: Option<Arc<BoundGlobalScope>>,
        variables: Vec<VariableSymbol>,
        expression: BoundExpression,
    ) -> Self {
        Self {
            previous,
            variables,
            expression,
        }
    }

    pub(crate) fn previous(&self) -> Option<&BoundGlobalScope> {
        self.previous.as_ref().map(|p| p.as_ref())
    }

    pub(crate) fn expression(&self) -> &BoundExpression {
        &self.expression
    }
}
