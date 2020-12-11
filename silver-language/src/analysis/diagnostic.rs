use super::diagnostic_kind::DiagnosticKind;
use super::text::text_span::TextSpan;

pub struct Diagnostic {
    span: TextSpan,
    message: String,
    kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(span: TextSpan, message: String, kind: DiagnosticKind) -> Self {
        Self {
            span,
            message,
            kind,
        }
    }

    pub fn span(&self) -> TextSpan {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
