use super::text::text_span::TextSpan;

pub struct Diagnostic {
    span: TextSpan,
    message: String,
}

impl Diagnostic {
    pub fn new(span: TextSpan, message: String) -> Self {
        Self { span, message }
    }

    pub fn span(&self) -> TextSpan {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
