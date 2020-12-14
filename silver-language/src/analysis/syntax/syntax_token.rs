use std::fmt::Display;

use crate::analysis::{silver_value::SilverValue, text::text_span::TextSpan};

use super::{syntax_kind::SyntaxKind, syntax_node::SyntaxNodeExt};

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    position: usize,
    text: String,
    value: Option<SilverValue>,
    span: TextSpan,
}

impl SyntaxNodeExt for SyntaxToken {
    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        vec![]
    }

    fn kind(&self) -> SyntaxKind {
        self.kind
    }

    fn value(&self) -> Option<&SilverValue> {
        self.value()
    }

    fn text(&self) -> Option<&str> {
        Some(&self.text)
    }

    fn span(&self) -> TextSpan {
        self.span()
    }
}

impl SyntaxToken {
    pub(crate) fn new(
        kind: SyntaxKind,
        position: usize,
        text: String,
        value: Option<SilverValue>,
    ) -> Self {
        Self {
            span: position..position + text.len(),
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
        &self.text
    }

    pub fn value(&self) -> Option<&SilverValue> {
        self.value.as_ref()
    }

    pub fn span(&self) -> TextSpan {
        self.span.clone()
    }
}

impl Display for SyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: '{}'", self.kind, self.text)?;
        if let Some(value) = &self.value {
            write!(f, " {}", value)?;
        }
        Ok(())
    }
}
