use super::error_reporter::ErrorReporter;
use crate::analysis::diagnostic::Diagnostic;

pub struct StringErrorReporter {
    errors: Vec<Diagnostic>,
}

impl StringErrorReporter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StringErrorReporter {
    fn default() -> Self {
        Self { errors: vec![] }
    }
}

impl ErrorReporter for StringErrorReporter {
    fn report_error(&mut self, error: Diagnostic) {
        self.errors.push(error);
    }

    fn had_error(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }

    fn clear(&mut self) {
        self.errors.clear();
    }
}
