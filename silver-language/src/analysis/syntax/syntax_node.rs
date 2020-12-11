use crate::analysis::silver_value::SilverValue;

use super::{expression_syntax::ExpressionSyntax, syntax_kind::SyntaxKind};

// TODO this enum won't be used for a while.
#[allow(dead_code)]
pub enum SyntaxNode<'source> {
    Expression(ExpressionSyntax<'source>),
}

pub trait SyntaxNodeExt {
    fn kind(&self) -> SyntaxKind;
    fn children(&self) -> Vec<&dyn SyntaxNodeExt>;
    fn value(&self) -> Option<&SilverValue>;
}

impl<'source> SyntaxNodeExt for SyntaxNode<'source> {
    fn kind(&self) -> SyntaxKind {
        SyntaxKind::Root
    }

    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        match self {
            SyntaxNode::Expression(e) => vec![e],
        }
    }

    fn value(&self) -> Option<&SilverValue> {
        // Only tokens have values
        None
    }
}
