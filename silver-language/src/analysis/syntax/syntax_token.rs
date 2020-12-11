use std::fmt::Display;

use crate::analysis::{silver_value::SilverValue, text::text_span::TextSpan};

use super::{syntax_kind::SyntaxKind, syntax_node::SyntaxNodeExt};

#[derive(Debug, Clone)]
pub struct SyntaxToken<'source> {
    kind: SyntaxKind,
    position: usize,
    text: &'source str,
    value: Option<SilverValue>,
}

impl<'source> SyntaxNodeExt for SyntaxToken<'source> {
    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        vec![]
    }

    fn kind(&self) -> SyntaxKind {
        self.kind
    }

    fn value(&self) -> Option<&SilverValue> {
        self.value()
    }
}

impl<'source> SyntaxToken<'source> {
    pub(crate) fn new(
        kind: SyntaxKind,
        position: usize,
        text: &'source str,
        value: Option<SilverValue>,
    ) -> Self {
        Self {
            kind,
            position,
            text,
            value,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn value(&self) -> Option<&SilverValue> {
        self.value.as_ref()
    }

    pub fn span(&self) -> TextSpan {
        self.position..self.position + self.text.len()
    }
}

impl<'source> Display for SyntaxToken<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: '{}'", self.kind, self.text)?;
        if let Some(value) = &self.value {
            write!(f, " {}", value)?;
        }
        Ok(())
    }
}
