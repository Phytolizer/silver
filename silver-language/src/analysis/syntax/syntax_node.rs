use crate::analysis::{silver_value::SilverValue, text::text_span::TextSpan};

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
    fn text(&self) -> Option<&str>;
    fn span(&self) -> TextSpan;
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

    fn text(&self) -> Option<&str> {
        // Only tokens have text
        None
    }

    fn span(&self) -> TextSpan {
        match self {
            SyntaxNode::Expression(e) => e.span(),
        }
    }
}

pub fn flatten_tree(root: &dyn SyntaxNodeExt) -> Vec<&dyn SyntaxNodeExt> {
    let mut stack = vec![];
    let mut out = vec![];

    stack.push(root);

    while let Some(n) = stack.pop() {
        out.push(n);

        for &child in n.children().iter().rev() {
            stack.push(child);
        }
    }

    out
}
