use crate::analysis::variable_symbol::VariableSymbol;

use super::{
    bound_expression::BoundExpression, bound_node::BoundNode, bound_node_kind::BoundNodeKind,
};

pub(crate) enum BoundStatement {
    VariableDeclaration {
        variable: VariableSymbol,
        initializer: BoundExpression,
    },
}

impl BoundNode for BoundStatement {
    fn kind(&self) -> super::bound_node_kind::BoundNodeKind {
        match self {
            BoundStatement::VariableDeclaration { .. } => {
                BoundNodeKind::VariableDeclarationStatement
            }
        }
    }

    fn children(&self) -> Vec<&dyn BoundNode> {
        match self {
            BoundStatement::VariableDeclaration {
                variable,
                initializer,
            } => vec![initializer],
        }
    }
}
